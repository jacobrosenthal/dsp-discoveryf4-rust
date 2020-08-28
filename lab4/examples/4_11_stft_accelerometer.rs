//! This project is used for explaining the STFT operation on real-world
//! signals. Here we first sample the acceleromater data. Sampling period is set
//! as 10 milliseconds. We also generate a Hamming window. These signals are
//! represented with x and v arrays in main.c file respectively. The input
//! signal is divided into subwindows and FFT of each subwindow is calculated by
//! the STFT function. The result is stored in the XST array.
//!
//! Requires `cargo install probe-run`
//! `cargo run --release --example 4_11_stft_accelerometer`
//!
//! Note: This is currently stack overflowing with Window larger than 16

#![no_std]
#![no_main]

use panic_break as _;
use stm32f4xx_hal as hal;

use cmsis_dsp_sys::{arm_cfft_f32, arm_cmplx_mag_f32};
use core::f32::consts::PI;
use cty::uint32_t;
use hal::{prelude::*, spi, stm32};
use itertools::Itertools;
use lis3dsh::{accelerometer::RawAccelerometer, Lis3dsh};
use micromath::F32Ext;
use rtt_target::{rprintln, rtt_init_print};

use cmsis_dsp_sys::arm_cfft_sR_f32_len16 as arm_cfft_sR_f32;
const N: usize = 1024;
const WINDOW: usize = 16;
const WINDOWCOMPLEX: usize = WINDOW * 2;

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

    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

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

    let chip_select = gpioe.pe3.into_push_pull_output();
    let mut lis3dsh = Lis3dsh::new_spi(spi, chip_select);
    lis3dsh.init(&mut delay).unwrap();

    rprintln!("reading accel");

    // dont love the idea of delaying in an iterator ...
    let accel = (0..N)
        .map(|_| {
            while !lis3dsh.is_data_ready().unwrap() {}
            let dat = lis3dsh.accel_raw().unwrap();
            dat[0] as f32
        })
        .collect::<heapless::Vec<f32, N>>();

    rprintln!("computing");

    let hamming = (0..WINDOW).map(|m| 0.54 - 0.46 * (2.0 * PI * m as f32 / WINDOW as f32).cos());

    // get 64 input at a time, overlapping 32
    // windowing is easier to do on slices
    let overlapping_chirp_windows = Windows {
        v: &accel[..],
        size: WINDOW,
        inc: WINDOW / 2,
    };

    let mut xst: heapless::Vec<[f32; WINDOW], N> = heapless::Vec::new();

    let mut mag = [0f32; WINDOW];

    for chirp_win in overlapping_chirp_windows {
        // 64-0=64 of input to 64-64=0, so input * chirp.rev
        let mut dtfsecoef = hamming
            .clone()
            .zip(chirp_win.iter().rev())
            .map(|(v, x)| v * x)
            .interleave_shortest(core::iter::repeat(0.0))
            .collect::<heapless::Vec<f32, WINDOWCOMPLEX>>();

        unsafe {
            //Finding the FFT of window
            arm_cfft_f32(&arm_cfft_sR_f32, dtfsecoef.as_mut_ptr(), 0, 1);
            arm_cmplx_mag_f32(dtfsecoef.as_ptr(), mag.as_mut_ptr(), WINDOW as uint32_t);
        }

        xst.push(mag).ok();
    }

    rprintln!("xst: {:?}", xst);

    // signal to probe-run to exit
    loop {
        cortex_m::asm::bkpt()
    }
}

pub struct Windows<'a, T: 'a> {
    v: &'a [T],
    size: usize,
    inc: usize,
}

impl<'a, T> Iterator for Windows<'a, T> {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<&'a [T]> {
        if self.size > self.v.len() {
            None
        } else {
            let ret = Some(&self.v[..self.size]);
            self.v = &self.v[self.inc..];
            ret
        }
    }
}

//C needs access to a sqrt fn, lets use micromath
#[no_mangle]
pub extern "C" fn sqrtf(x: f32) -> f32 {
    x.sqrt()
}
