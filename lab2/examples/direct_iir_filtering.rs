//! This project is used for explaining IIR filtering operation using constant
//! coefficient difference equation.
//!
//! Requires cargo embed
//! `cargo install cargo-embed`
//!
//! `cargo embed --release --example direct_iir_filtering`

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

static B: &'static [f32] = &[0.002044, 0.004088, 0.002044];
static A: &'static [f32] = &[1f32, -1.819168, 0.827343];

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
    x.iter_mut()
        .enumerate()
        .for_each(|(n, val)| *val = (PI * n as f32 / 128f32).sin() + (FRAC_PI_4 * n as f32).sin());

    let mut y = [0f32; N];
    //random access of &mut y were iterating over.. so no iterators unless ... todo
    for y_idx in 0..N {
        y[y_idx] = B
            .iter()
            .enumerate()
            .map(|(coeff_idx, coeff)| {
                if let Some(idx) = y_idx.checked_sub(coeff_idx) {
                    coeff * x[idx]
                } else {
                    0f32
                }
            })
            .sum::<f32>()
            + A.iter()
                .enumerate()
                .map(|(coeff_idx, coeff)| {
                    if let Some(idx) = y_idx.checked_sub(coeff_idx) {
                        -(coeff * y[idx])
                    } else {
                        0f32
                    }
                })
                .sum::<f32>();
    }
    dbgprint!("y: {:?}", &y[..]);

    loop {}
}
