//! This project is used for creating five different basic digital signals: unit
//! pulse, unit step, unit ramp, exponential and sinusoidal.
//!
//! Requires cargo embed
//! `cargo install cargo-embed`
//!
//! `cargo embed --example 2_9`

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

const N: usize = 100;
const SAW_AMPLITUDE: f32 = 0.75;
const SAW_PERIOD: usize = 20;

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

    // One period of the sawtooth signal
    let mut sawtooth = [0f32; N];
    sawtooth.iter_mut().enumerate().for_each(|(idx, val)| {
        *val = (2.0 * SAW_AMPLITUDE / (SAW_PERIOD as f32 - 1.0)) * idx as f32 - SAW_AMPLITUDE;
    });
    dbgprint!("sawtooth period: {:?}", &sawtooth[..]);

    // Generating the sawtooth signal
    for idx in 0..N {
        sawtooth[idx] = sawtooth[idx % SAW_PERIOD];
    }
    dbgprint!("sawtooth signal: {:?}", &sawtooth[..]);

    loop {}
}
