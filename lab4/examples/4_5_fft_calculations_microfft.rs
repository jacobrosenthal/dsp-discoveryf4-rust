//! This project is used for explaining the FFT operation using ARM CMSIS-DSP
//! library functions. Here we have a digital input signal, sum of two
//! sinusoidal signals with different frequencies. The complex form of this
//! signal is represented with s_complex array in main.c file. Frequency
//! components of this signal are found with arm_cfft_f32 function. Output of
//! this function is saved in the input array. The magnitude of the output
//! signal is calculated with the arm_cmplx_mag_f32 function. The result is
//! saved in the Mag array.
//!
//! Requires `cargo install probe-run`
//! `cargo run --release --example 4_5_fft_calculations_microfft`

#![no_std]
#![no_main]

use panic_probe as _;
use stm32f4xx_hal as hal;

use hal::{dwt::ClockDuration, dwt::DwtExt, prelude::*, stm32};
use microfft::Complex32;
use micromath::F32Ext;
use rtt_target::{rprintln, rtt_init_print};

use microfft::complex::cfft_256 as cfft;
const N: usize = 256;

const W1: f32 = core::f32::consts::PI / 128.0;
const W2: f32 = core::f32::consts::PI / 4.0;
// const W2: f32 = core::f32::consts::PI / 5.0;

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

    // Create a delay abstraction based on DWT cycle counter
    let dwt = cp.DWT.constrain(cp.DCB, clocks);

    // Complex sum of sinusoidal signals
    let s1 = (0..N).map(|val| (W1 * val as f32).sin());
    let s2 = (0..N).map(|val| (W2 * val as f32).sin());
    let s = s1.zip(s2).map(|(ess1, ess2)| ess1 + ess2);

    // map it to real, leave im blank well fill in with dft
    let mut dtfsecoef: heapless::Vec<Complex32, N> =
        s.map(|f| Complex32 { re: f, im: 0.0 }).collect();

    let time: ClockDuration = dwt.measure(|| {
        // SAFETY microfft now only accepts arrays instead of slices to avoid runtime errors
        // Thats not great for us. However we can cheat since our slice into an array because
        // "The layout of a slice [T] of length N is the same as that of a [T; N] array."
        // https://rust-lang.github.io/unsafe-code-guidelines/layout/arrays-and-slices.html
        // this goes away when something like heapless vec is in standard library
        // https://github.com/rust-lang/rfcs/pull/2990
        unsafe {
            let ptr = &mut *(dtfsecoef.as_mut_ptr() as *mut [Complex32; N]);

            // Coefficient calculation with CFFT function
            // well use microfft uses an in place Radix-2 FFT
            // it re-returns our array in case we were going to chain calls, throw it away
            let _ = cfft(ptr);
        }

        // Magnitude calculation
        let _mag: heapless::Vec<f32, N> = dtfsecoef
            .iter()
            .map(|complex| (complex.re * complex.re + complex.im * complex.im).sqrt())
            .collect();
    });
    rprintln!("ticks: {:?}", time.as_ticks());

    // signal to probe-run to exit
    loop {
        cortex_m::asm::bkpt()
    }
}
