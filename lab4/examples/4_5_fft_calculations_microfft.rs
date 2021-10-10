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
#![feature(array_from_fn)]

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

    // some sensor data source collected to an array so often
    // Complex sum of sinusoidal signals
    let s: [f32; N] = core::array::from_fn(|n| (W1 * n as f32).sin() + (W2 * n as f32).sin());

    // Use Complex32 to interleave 0.0 for imaginary
    let mut dtfsecoef: [Complex32; N] = s.map(|v| Complex32 { re: v, im: 0.0 });

    let mut mag = [0f32; N];

    let time: ClockDuration = dwt.measure(|| {
        // Coefficient calculation with CFFT function
        // well use microfft uses an in place Radix-2 FFT
        let _ = cfft(&mut dtfsecoef);

        // Magnitude calculation
        for (v, complex) in mag.iter_mut().zip(dtfsecoef) {
            *v = (complex.re * complex.re + complex.im * complex.im).sqrt()
        }
    });

    rprintln!("mag: {:?}", mag);
    rprintln!("ticks: {:?}", time.as_ticks());

    // signal to probe-run to exit
    loop {
        cortex_m::asm::bkpt()
    }
}
