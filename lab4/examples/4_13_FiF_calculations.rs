//! This project is used for explaining filtering in frequency domain. Here, we
//! have a digital input signal as the sum of two sinusoids with different
//! frequencies. The complex form of this signal is represented with s_complex
//! array in main.c file. Also we have a digital filter represented with h array
//! given in FIR_lpf_coefficients.h file.
//!
//! Requires cargo embed `cargo install cargo-embed`
//!
//! `cargo embed --example 4_13_FiF_calculations`

#![no_std]
#![no_main]

use stm32f4xx_hal as hal;

use crate::hal::{dwt::ClockDuration, dwt::DwtExt, prelude::*, stm32};
use cortex_m_rt::entry;
use jlink_rtt;
use micromath::F32Ext;
use panic_rtt as _;

macro_rules! dbgprint {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut out = $crate::jlink_rtt::NonBlockingOutput::new();
            writeln!(out, $($arg)*).ok();
        }
    };
}

use microfft::{complex::cfft_512, Complex32};
use typenum::Unsigned;

type N = heapless::consts::U512;

const W1: f32 = core::f32::consts::PI / 128.0;
const W2: f32 = core::f32::consts::PI / 4.0;

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

    // Create a delay abstraction based on DWT cycle counter
    let dwt = cp.DWT.constrain(cp.DCB, clocks);

    // Complex sum of sinusoidal signals
    let s1 = (0..N::to_usize()).map(|val| (W1 * val as f32).sin());
    let s2 = (0..N::to_usize()).map(|val| (W2 * val as f32).sin());
    let s = s1.zip(s2).map(|(ess1, ess2)| ess1 + ess2);

    // map it to real, leave im blank well fill in with cfft
    let mut dtfsecoef = s
        .clone()
        .map(|f| Complex32 { re: f, im: 0.0 })
        .collect::<heapless::Vec<Complex32, N>>();

    // Complex impulse response of filter
    let mut df_complex = H
        .iter()
        .cloned()
        .map(|f| Complex32 { re: f, im: 0.0 })
        .collect::<heapless::Vec<Complex32, N>>();

    // Finding the FFT of the filter
    let _ = cfft_512(&mut df_complex[..]);

    let time: ClockDuration = dwt.measure(|| {
        // Finding the FFT of the input signal
        let _ = cfft_512(&mut dtfsecoef[..]);

        // Filtering in the frequency domain
        // arm_cmplx_mult_cmplx_f32(s_complex, df_complex, y_complex, 2 * N);
        let mut y_complex = ;

        // Finding the complex result in time domain
        // arm_cfft_f32(&arm_cfft_sR_f32_len512, y_complex, 1, 1);
        // Uh oh, 1,1 ... 
        let _ = cfft_512(&mut y_complex[..]);

    });
    dbgprint!("dft ticks: {:?}", time.as_ticks());

    loop {}
}

static H: &'static [f32] = &[
    0.002044, 0.007806, 0.014554, 0.020018, 0.024374, 0.027780, 0.030370, 0.032264, 0.033568,
    0.034372, 0.034757, 0.034791, 0.034534, 0.034040, 0.033353, 0.032511, 0.031549, 0.030496,
    0.029375, 0.028207, 0.027010, 0.025800, 0.024587, 0.023383, 0.022195, 0.021031, 0.019896,
    0.018795, 0.017730, 0.016703, 0.015718, 0.014774, 0.013872, 0.013013, 0.012196, 0.011420,
    0.010684, 0.009989, 0.009331, 0.008711, 0.008127, 0.007577, 0.007061, 0.006575, 0.006120,
    0.005693, 0.005294, 0.004920, 0.004570, 0.004244, 0.003939, 0.003655, 0.003389, 0.003142,
    0.002912, 0.002698, 0.002499, 0.002313, 0.002141, 0.001981, 0.001833, 0.001695, 0.001567,
    0.001448,
];