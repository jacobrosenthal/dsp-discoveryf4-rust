//! This project is used to measure the code execution in terms of clock cycles.
//!
//! With cargo embed `cargo install cargo-embed`
//!
//! `cargo embed --release --example cycle_count`

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use jlink_rtt;
use panic_rtt as _;
use stm32f4xx_hal::{dwt::ClockDuration, dwt::DwtExt, prelude::*, stm32};

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
    let mut delay = dwt.delay();

    let time: ClockDuration = dwt.measure(|| delay.delay_ms(100_u32));
    dbgprint!("ticks: {:?}", time.as_ticks());

    loop {}
}
