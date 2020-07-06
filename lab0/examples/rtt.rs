//! Demo logging via rtt. Panic's are also fed through rtt.
//!
//! With cargo embed
//! `cargo install cargo-embed`
//!
//! `cargo embed --release --example rtt`

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_rtt as _;
use stm32f4xx_hal as _;

macro_rules! dbgprint {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut out = jlink_rtt::Output::new();
            writeln!(out, $($arg)*).ok();
        }
    };
}

#[entry]
fn main() -> ! {
    dbgprint!("main entered");
    panic!("ded");
}
