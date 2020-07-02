//! This project is used for explaining IIR filtering operation using constant
//! coefficient difference equation.
//!
//! Requires cargo embed
//! `cargo install cargo-embed`
//!
//! `cargo embed --example 2_23_direct_iir_filtering`

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
use typenum::Unsigned;

type N = heapless::consts::U512;
const N_CONST: usize = 512;

// high pass filter coefficients
static B: &'static [f32] = &[0.002044, 0.004088, 0.002044];
static A: &'static [f32] = &[1.0, -1.819168, 0.827343];

// low pass filter coefficients for 2_24
// static B: &'static [f32] = &[0.705514, -1.411028, 0.705514];
// static A: &'static [f32] = &[1.0, -1.359795, 0.462261];

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

    let x = (0..N::to_usize())
        .map(|n| (PI * n as f32 / 128.0).sin() + (FRAC_PI_4 * n as f32).sin())
        .collect::<heapless::Vec<f32, N>>();

    //random access of &mut y were iterating over.. so no iterators unless ... todo
    let mut y = [0.0; N_CONST];
    for y_n in 0..N_CONST {
        y[y_n] = B
            .iter()
            .enumerate()
            .map(|(coeff_n, coeff)| {
                if coeff_n < (y_n + 1) {
                    coeff * x[y_n - coeff_n]
                } else {
                    0.0
                }
            })
            .sum::<f32>()
            + A.iter()
                .enumerate()
                .map(|(coeff_n, coeff)| {
                    if coeff_n < (y_n + 1) {
                        -(coeff * y[y_n - coeff_n])
                    } else {
                        0.0
                    }
                })
                .sum::<f32>();
    }
    dbgprint!("y: {:?}", &y[..]);

    loop {}
}
