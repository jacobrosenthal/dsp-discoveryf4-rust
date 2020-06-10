//! This project is used for creating five different basic digital signals: unit
//! pulse, unit step, unit ramp, exponential and sinusoidal.
//!
//! Requires cargo embed
//! `cargo install cargo-embed`
//!
//! `cargo embed --release --example basic_signals`

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

use micromath::F32Ext;

const N: usize = 10;
const A: f32 = 0.8;
const W0: f32 = core::f32::consts::PI / 5f32;

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

    //unit pulse signal
    let mut unit_pulse = [0; N];
    unit_pulse.iter_mut().enumerate().for_each(|(idx, val)| {
        if idx == 0 {
            *val = 1;
        } else {
            *val = 0;
        }
    });
    dbgprint!("unit_pulse: {:?}", &unit_pulse[..]);

    //unit step signal
    let unit_step = [1; N];
    dbgprint!("unit_step: {:?}", &unit_step[..]);

    //unit ramp signal
    let mut unit_ramp: [i32; N] = [0; N];
    unit_ramp
        .iter_mut()
        .enumerate()
        .for_each(|(idx, val)| *val = idx as i32);
    dbgprint!("unit_ramp: {:?}", &unit_ramp[..]);

    //exponential signal
    let mut exponential = [0f32; N];
    exponential
        .iter_mut()
        .enumerate()
        .for_each(|(idx, val)| *val = A.powf(idx as f32));
    dbgprint!("exponential: {:?}", &exponential);

    //sinusoidal signal
    let mut sinusoidal = [0f32; N];
    sinusoidal
        .iter_mut()
        .enumerate()
        .for_each(|(idx, val)| *val = (W0 * idx as f32).sin());
    dbgprint!("sinusoidal: {:?}", &sinusoidal);

    loop {}
}
