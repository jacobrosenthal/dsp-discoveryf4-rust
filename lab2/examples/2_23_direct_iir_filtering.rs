//! This project is used for explaining IIR filtering operation using constant
//! coefficient difference equation.
//!
//! Requires `cargo install probe-run`
//! `cargo run --release --example 2_23_direct_iir_filtering`

#![no_std]
#![no_main]

use panic_break as _;
use stm32f4xx_hal as hal;

use core::f32::consts::{FRAC_PI_4, PI};
use hal::{prelude::*, stm32};
use micromath::F32Ext;
use rtt_target::{rprintln, rtt_init_print};

const N: usize = 512;

// high pass filter coefficients
static B: &[f32] = &[0.002044, 0.004088, 0.002044];
static A: &[f32] = &[1.0, -1.819168, 0.827343];

// low pass filter coefficients for 2_24
// static B: &[f32] = &[0.705514, -1.411028, 0.705514];
// static A: &[f32] = &[1.0, -1.359795, 0.462261];

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!(BlockIfFull, 128);

    let dp = stm32::Peripherals::take().unwrap();
    let _cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // Set up the system clock.
    let rcc = dp.RCC.constrain();

    let _clocks = rcc
        .cfgr
        .use_hse(8.mhz()) //discovery board has 8 MHz crystal for HSE
        .sysclk(168.mhz())
        .freeze();

    let x: heapless::Vec<f32, N> = (0..N)
        .map(|n| (PI * n as f32 / 128.0).sin() + (FRAC_PI_4 * n as f32).sin())
        .collect();

    //random access of &mut y were iterating over.. so no iterators unless
    let mut y = [0.0; N];
    for y_n in 0..N {
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

    rprintln!("y: {:?}", y);

    // signal to probe-run to exit
    loop {
        cortex_m::asm::bkpt()
    }
}
