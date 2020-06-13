//! This project is used for creating five different basic digital signals: unit
//! pulse, unit step, unit ramp, exponential and sinusoidal. 2_1
//!
//! Heapless vec is a vector with a fixed capacity of U elements allocated on
//! the stack. This solution will become necessary as we continue to avoid
//! allocating. However its not ideal as we carray around extra type parameters
//! for the length of the vec. Further these are double defined because were
//! also dealing with a traditional const N length as well... This gets better
//! with const generics arrive someday.
//!
//! Requires cargo embed `cargo install cargo-embed`
//!
//! `cargo embed --example 2_1_heapless_basic_signals`

#![no_std]
#![no_main]

use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, stm32};
use cortex_m_rt::entry;
use jlink_rtt;
use panic_rtt as _;

use heapless::consts::U10;
const N: usize = 10;

mod utils;

macro_rules! dbgprint {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut out = $crate::jlink_rtt::NonBlockingOutput::new();
            writeln!(out, $($arg)*).ok();
        }
    };
}

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

    let unit_pulse = utils::unit_pulse(0..N).collect::<heapless::Vec<f32, U10>>();
    dbgprint!("unit_pulse: {:?}", &unit_pulse[..]);

    let unit_step = utils::unit_step(0..N).collect::<heapless::Vec<f32, U10>>();
    dbgprint!("unit_step: {:?}", &unit_step[..]);

    let unit_ramp = utils::unit_ramp(0..N).collect::<heapless::Vec<f32, U10>>();
    dbgprint!("unit_ramp: {:?}", &unit_ramp[..]);

    let exponential = utils::exponential(0..N).collect::<heapless::Vec<f32, U10>>();
    dbgprint!("exponential: {:?}", &exponential[..]);

    let sinusoidal = utils::sinusoidal(0..N).collect::<heapless::Vec<f32, U10>>();
    dbgprint!("sinusoidal: {:?}", &sinusoidal[..]);

    loop {}
}
