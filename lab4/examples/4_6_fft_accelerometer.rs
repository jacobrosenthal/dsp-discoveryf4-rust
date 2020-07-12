//! This project is used for explaining the FFT operation on real-world signals.
//! Here we first sample the acceleromater data. The sampling period is set as
//! 10 milliseconds. The complex form of this signal is stored in X array.
//! Frequency components of this signal are obtained with the arm_cfft_f32
//! function. Output of this function is saved in the input array. Magnitude of
//! the output signal is calculated with the arm_cmplx_mag_f32 function. The
//! result is saved in the Mag array.
//!
//! Requires cargo embed `cargo install cargo-embed`
//!
//! `cargo embed --example 4_6_fft_accelerometer`

#![no_std]
#![no_main]

use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, spi, stm32};
use cortex_m_rt::entry;
use micromath::F32Ext;
use panic_rtt as _;

macro_rules! dbgprint {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut out = jlink_rtt::Output::new();
            writeln!(out, $($arg)*).ok();
        }
    };
}

use accelerometer::RawAccelerometer;
use cmsis_dsp_sys::{arm_cfft_f32, arm_cfft_sR_f32_len512, arm_cmplx_mag_f32};
use cty::uint32_t;
use itertools::Itertools;
use lis302dl::Lis302Dl;
use typenum::{Sum, Unsigned};

type N = heapless::consts::U512;
type NCOMPLEX = Sum<N, N>;
//todo derive this from N
const N_CONST: usize = 512;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // Set up the system clock.
    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz()) //discovery board has 8 MHz crystal for HSE
        .sysclk(168.mhz())
        .freeze();

    let gpioa = dp.GPIOA.split();
    let gpioe = dp.GPIOE.split();

    let sck = gpioa.pa5.into_alternate_af5().internal_pull_up(false);
    let miso = gpioa.pa6.into_alternate_af5().internal_pull_up(false);
    let mosi = gpioa.pa7.into_alternate_af5().internal_pull_up(false);

    let spi_mode = spi::Mode {
        polarity: spi::Polarity::IdleLow,
        phase: spi::Phase::CaptureOnFirstTransition,
    };

    let spi = spi::Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        spi_mode,
        10.mhz().into(),
        clocks,
    );

    let mut chip_select = gpioe.pe3.into_push_pull_output();
    chip_select.set_high().ok();

    let mut lis302dl = Lis302Dl::new(spi, chip_select, Default::default());

    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    // dont love the idea of delaying in an iterator ...
    let dtfsecoef = (0..N::to_usize()).map(|_| {
        delay.delay_ms(10u8);
        let dat = lis302dl.accel_raw().unwrap();
        dat.x as f32
    });

    let mut dtfsecoef = dtfsecoef
        .interleave_shortest(core::iter::repeat(0.0))
        .collect::<heapless::Vec<f32, NCOMPLEX>>();

    let mut mag = [0f32; N_CONST];

    unsafe {
        //CFFT calculation
        arm_cfft_f32(&arm_cfft_sR_f32_len512, dtfsecoef.as_mut_ptr(), 0, 1);

        // Magnitude calculation
        arm_cmplx_mag_f32(
            dtfsecoef.as_ptr(),
            mag.as_mut_ptr(),
            N::to_usize() as uint32_t,
        );
    }

    dbgprint!("mag: {:?}", &mag[..]);

    loop {}
}

//C needs access to a sqrt fn, lets use micromath
#[no_mangle]
pub extern "C" fn sqrtf(x: f32) -> f32 {
    x.sqrt()
}
