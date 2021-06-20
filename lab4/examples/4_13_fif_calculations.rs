//! This project is used for explaining filtering in frequency domain. Here, we
//! have a digital input signal as the sum of two sinusoids with different
//! frequencies. The complex form of this signal is represented with s_complex
//! array in main.c file. Also we have a digital filter represented with h array
//! given in FIR_lpf_coefficients.h file.
//!
//! Requires `cargo install probe-run`
//! `cargo run --release --example 4_13_fif_calculations`

#![no_std]
#![no_main]

use panic_break as _;
use stm32f4xx_hal as hal;

use cmsis_dsp_sys::{arm_cfft_f32, arm_cmplx_mult_cmplx_f32};
use cty::uint32_t;
use hal::{dwt::ClockDuration, dwt::DwtExt, prelude::*, stm32};
use itertools::Itertools;
use micromath::F32Ext;
use rtt_target::{rprintln, rtt_init_print};

use cmsis_dsp_sys::arm_cfft_sR_f32_len512 as arm_cfft_sR_f32;
const N: usize = 512;
const NCOMPLEX: usize = N * 2;

const W1: f32 = core::f32::consts::PI / 128.0;
const W2: f32 = core::f32::consts::PI / 4.0;

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

    //we wont use complex this time since, but just interleave the zeros for the imaginary part
    let mut s_complex: heapless::Vec<f32, NCOMPLEX> = s1
        .zip(s2)
        .map(|(ess1, ess2)| ess1 + ess2)
        .interleave_shortest(core::iter::repeat(0.0))
        .collect();

    // Complex impulse response of filter
    let mut df_complex: heapless::Vec<f32, NCOMPLEX> = H
        .iter()
        .cloned()
        .interleave_shortest(core::iter::repeat(0.0))
        .chain(core::iter::repeat(0.0))
        //fill rest with zeros up to N*2
        .take(NCOMPLEX)
        .collect();

    // Finding the FFT of the filter
    unsafe {
        arm_cfft_f32(&arm_cfft_sR_f32, df_complex.as_mut_ptr(), 0, 1);
    }

    let mut y_complex = [0f32; N * 2];

    let time: ClockDuration = dwt.measure(|| {
        // Finding the FFT of the input signal
        unsafe {
            arm_cfft_f32(&arm_cfft_sR_f32, s_complex.as_mut_ptr(), 0, 1);
        }

        // Filtering in the frequency domain
        unsafe {
            arm_cmplx_mult_cmplx_f32(
                s_complex.as_ptr(),
                df_complex.as_ptr(),
                y_complex.as_mut_ptr(),
                N as uint32_t,
            );
        }

        // Finding the complex result in time domain
        unsafe {
            arm_cfft_f32(&arm_cfft_sR_f32, y_complex.as_mut_ptr(), 1, 1);
        }
    });
    rprintln!("dft ticks: {:?}", time.as_ticks());

    // signal to probe-run to exit
    loop {
        cortex_m::asm::bkpt()
    }
}

static H: &[f32] = &[
    0.002044, 0.007806, 0.014554, 0.020018, 0.024374, 0.027780, 0.030370, 0.032264, 0.033568,
    0.034372, 0.034757, 0.034791, 0.034534, 0.034040, 0.033353, 0.032511, 0.031549, 0.030496,
    0.029375, 0.028207, 0.027010, 0.025800, 0.024587, 0.023383, 0.022195, 0.021031, 0.019896,
    0.018795, 0.017730, 0.016703, 0.015718, 0.014774, 0.013872, 0.013013, 0.012196, 0.011420,
    0.010684, 0.009989, 0.009331, 0.008711, 0.008127, 0.007577, 0.007061, 0.006575, 0.006120,
    0.005693, 0.005294, 0.004920, 0.004570, 0.004244, 0.003939, 0.003655, 0.003389, 0.003142,
    0.002912, 0.002698, 0.002499, 0.002313, 0.002141, 0.001981, 0.001833, 0.001695, 0.001567,
    0.001448,
];

//C needs access to a sqrt fn, lets use micromath
#[no_mangle]
pub extern "C" fn sqrtf(x: f32) -> f32 {
    x.sqrt()
}
