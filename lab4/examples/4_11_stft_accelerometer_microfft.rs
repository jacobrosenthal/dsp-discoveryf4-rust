//! This project is used for explaining the STFT operation on real-world
//! signals. Here we first sample the acceleromater data. Sampling period is set
//! as 10 milliseconds. We also generate a Hamming window. These signals are
//! represented with x and v arrays in main.c file respectively. The input
//! signal is divided into subwindows and FFT of each subwindow is calculated by
//! the STFT function. The result is stored in the XST array.
//!
//! Requires `cargo install probe-run`
//! `cargo run --release --example 4_11_stft_accelerometer_microfft`
//!
//! Note: This is currently stack overflowing with Window larger than 16

#![no_std]
#![no_main]

use panic_probe as _;
use stm32f4xx_hal as hal;

use core::f32::consts::PI;
use hal::{prelude::*, spi, stm32};
use lis3dsh::{accelerometer::RawAccelerometer, Lis3dsh};
use microfft::Complex32;
use micromath::F32Ext;
use rtt_target::{rprintln, rtt_init_print};

use microfft::complex::cfft_16 as cfft;
const WINDOW: usize = 16;

const N: usize = 1024;
const NDIV2: usize = N / 2;

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

    let spi = spi::Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        spi::Mode {
            polarity: spi::Polarity::IdleLow,
            phase: spi::Phase::CaptureOnFirstTransition,
        },
        10.mhz().into(),
        clocks,
    );

    let chip_select = gpioe.pe3.into_push_pull_output();
    let mut lis3dsh = Lis3dsh::new_spi(spi, chip_select);
    lis3dsh.init(&mut delay).unwrap();

    rprintln!("reading accel");

    // dont love the idea of delaying in an iterator ...
    let accel: heapless::Vec<f32, N> = (0..N)
        .map(|_| {
            while !lis3dsh.is_data_ready().unwrap() {}
            let dat = lis3dsh.accel_raw().unwrap();
            dat[0] as f32
        })
        .collect();

    rprintln!("computing");

    let hamming = (0..WINDOW).map(|m| 0.54 - 0.46 * (2.0 * PI * m as f32 / WINDOW as f32).cos());

    // get 64 input at a time, overlapping 32
    // windowing is easier to do on slices
    let overlapping_chirp_windows = Windows {
        v: &accel,
        size: WINDOW,
        inc: WINDOW / 2,
    };

    let xst: heapless::Vec<_, NDIV2> = overlapping_chirp_windows
        .map(|chirp_win| {
            let mut dtfsecoef: heapless::Vec<Complex32, WINDOW> = hamming
                .clone()
                .zip(chirp_win.iter().rev())
                .map(|(v, x)| Complex32 { re: v * x, im: 0.0 })
                .collect();

            // SAFETY microfft now only accepts arrays instead of slices to avoid runtime errors
            // Thats not great for us. However we can cheat since our slice into an array because
            // "The layout of a slice [T] of length N is the same as that of a [T; N] array."
            // https://rust-lang.github.io/unsafe-code-guidelines/layout/arrays-and-slices.html
            // this goes away when something like heapless vec is in standard library
            // https://github.com/rust-lang/rfcs/pull/2990
            unsafe {
                let ptr = &mut *(dtfsecoef.as_mut_ptr() as *mut [Complex32; WINDOW]);

                // Coefficient calculation with CFFT function
                // well use microfft uses an in place Radix-2 FFT
                // it re-returns our array in case we were going to chain calls, throw it away
                let _ = cfft(ptr);
            }

            // Magnitude calculation
            let mag: heapless::Vec<_, WINDOW> = dtfsecoef
                .iter()
                .map(|complex| (complex.re * complex.re + complex.im * complex.im).sqrt())
                .collect();
            mag
        })
        .collect();

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
