//! Demo logging via rtt. Panic's are also fed through rtt.
//!
//! With cargo embed
//! `cargo install cargo-embed`
//!
//! `cargo embed --release --example rtt`

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal as _;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("main entered");
    panic!("ded");
}
