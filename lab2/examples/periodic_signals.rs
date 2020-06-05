//! This project is used for creating two different digital signals.
//! One of these signals is a periodic cosine wave and other one is aperiodic cosine wave.
//!
//! Requires cargo embed
//! `cargo install cargo-embed`
//!
//! `cargo embed --release --example periodic_signals`

#![no_std]
#![no_main]

use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, stm32};
pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
use cortex_m_rt::entry;
use panic_halt as _;

use micromath::F32Ext;

const N: usize = 100;
const W1: f32 = core::f32::consts::PI / 10f32;
const W2: f32 = 3f32 / 10f32;

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

    let mut sinusoidal1 = [0f32; N];
    sinusoidal1
        .iter_mut()
        .enumerate()
        .for_each(|(idx, val)| *val = (W1 * (idx as f32)).cos());

    let mut sinusoidal2 = [0f32; N];
    sinusoidal2
        .iter_mut()
        .enumerate()
        .for_each(|(idx, val)| *val = (W2 * (idx as f32)).cos());

    loop {}
}
