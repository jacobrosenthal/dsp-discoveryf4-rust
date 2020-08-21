//! This project is used for explaining the FFT operation on real-world signals.
//! Here we first sample the acceleromater data. The sampling period is set as
//! 10 milliseconds. The complex form of this signal is stored in X array.
//! Frequency components of this signal are obtained with the arm_cfft_f32
//! function. Output of this function is saved in the input array. Magnitude of
//! the output signal is calculated with the arm_cmplx_mag_f32 function. The
//! result is saved in the Mag array.
//!
//! Requires `cargo install probe-run`
//! `cargo run --release --example 4_6_fft_accelerometer_microfft`

#![no_std]
#![no_main]

use panic_break as _;
use stm32f4xx_hal as hal;

use accelerometer::RawAccelerometer;
use hal::{prelude::*, spi, stm32};
use lis302dl::Lis302Dl;
use microfft::{complex::cfft_512, Complex32};
use micromath::F32Ext;
use rtt_target::{rprintln, rtt_init_print};
use typenum::Unsigned;

type N = heapless::consts::U512;

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!(BlockIfFull, 128);

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
    let mut dtfsecoef = (0..N::to_usize())
        .map(|_| {
            let dat = lis302dl.accel_raw().unwrap();
            delay.delay_ms(10u8);

            Complex32 {
                re: dat.x as f32,
                im: 0.0,
            }
        })
        .collect::<heapless::Vec<Complex32, N>>();

    let _ = cfft_512(&mut dtfsecoef[..]);

    // Magnitude calculation
    let mag = dtfsecoef
        .iter()
        .map(|complex| (complex.re * complex.re + complex.im * complex.im).sqrt())
        .collect::<heapless::Vec<f32, N>>();

    rprintln!("mag: {:?}", &mag[..]);

    // signal to probe-run to exit
    loop {
        cortex_m::asm::bkpt()
    }
}