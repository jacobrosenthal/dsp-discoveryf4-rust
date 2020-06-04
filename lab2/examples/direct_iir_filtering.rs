//! Led Blinky Roulette example using the DWT peripheral for timing.
//!
//! This project is used for creating five different basic digital signals: unit pulse, unit step, unit ramp, exponential and sinusoidal. These signals are represented with d1, u1, r, e1 and s arrays in main.rs file.
//!
//! Open this project in Keil, debug it and run the code as explained in Lab 0 of the lab manual. Then you can export these five arrays using Export.ini file as explained in Section 0.4.3 of the lab manual. This file is already available in the project folder.  
//!
//! Requires cargo flash
//!
//! `cargo install cargo-flash`
//!
//! `cargo flash --example roulette --release --chip STM32F407VGTx --protocol swd`

#![no_std]
#![no_main]

use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, stm32};
pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
use cortex_m_rt::entry;
use panic_halt as _;

use core::f32::consts::{FRAC_PI_4, PI};
use micromath::F32Ext;

const N: usize = 512;

static B: &'static [f32] = &[0.002044, 0.004088, 0.002044];
static A: &'static [f32] = &[1f32, -1.819168, 0.827343];

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // Set up the system clock.
    let rcc = dp.RCC.constrain();

    let _clocks = rcc
        .cfgr
        .use_hse(8.mhz()) //discovery board has 8 MHz crystal for HSE
        .sysclk(168.mhz())
        .freeze();

    let mut itm = cp.ITM;

    iprintln!(&mut itm.stim[0], "Hello, world!");

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

    loop {}
}
