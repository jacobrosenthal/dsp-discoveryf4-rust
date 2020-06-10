//! This project is used for explaining FIR filtering operation using
//! convolution sum operation.
//!
//! Requires cargo embed
//! `cargo install cargo-embed`
//!
//! `cargo embed --release --example direct_fir_filtering`

#![no_std]
#![no_main]

use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, stm32};
use cortex_m_rt::entry;
use jlink_rtt;
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

use core::f32::consts::{FRAC_PI_4, PI};
use micromath::F32Ext;

const N: usize = 512;

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

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let _cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // Set up the system clock.
    let rcc = dp.RCC.constrain();

    let _clocks = rcc
        .cfgr
        .use_hse(8.mhz()) //discovery board has 8 MHz crystal for HSE
        .sysclk(168.mhz())
        .freeze();

    let mut x = [0f32; N];
    x.iter_mut().enumerate().for_each(|(idx, val)| {
        *val = (PI * idx as f32 / 128.0).sin() + (FRAC_PI_4 * idx as f32).sin()
    });
    dbgprint!("x: {:?}", &x[..]);

    //convolution_sum on x
    //cant be a map or iterator adapter on x because random access
    let mut y = [0f32; N];
    y.iter_mut().enumerate().for_each(|(y_idx, y_ref)| {
        *y_ref = H
            .iter()
            .enumerate()
            .map(|(coeff_idx, coeff)| {
                if let Some(x_idx) = y_idx.checked_sub(coeff_idx) {
                    coeff * x[x_idx]
                } else {
                    0.0
                }
            })
            .sum()
    });
    dbgprint!("y: {:?}", &y[..]);

    loop {}
}
