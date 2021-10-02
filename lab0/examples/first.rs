//! Demo iterators.
//!
//! Requires `cargo install cargo-embed`
//!
//! cargo-embed builds and uploads your code and maintains a two way connection
//! which in combination with panic-probe and rtt-target lets you send and
//! receive debug data over long lasting sessions.
//!
//! `cargo embed --example first --release`

#![no_std]
#![no_main]
#![feature(array_from_fn)]

use panic_probe as _;
use stm32f4xx_hal as hal;

use hal::{prelude::*, stm32};
use rtt_target::{rprintln, rtt_init_print};

static A: [i32; 5] = [1, 2, 3, 4, 5];
static B: [i32; 5] = [1, 2, 3, 4, 5];
const LEN: usize = 5;

#[cortex_m_rt::entry]
fn main() -> ! {
    // allocate the rtt machinery for printing
    rtt_init_print!(BlockIfFull, 128);

    let dp = stm32::Peripherals::take().unwrap();

    // Set up the system clock.
    let rcc = dp.RCC.constrain();

    let _ = rcc
        .cfgr
        .use_hse(8.mhz()) //discovery board has 8 MHz crystal for HSE
        .sysclk(168.mhz())
        .freeze();

    // We cant use vec in no_std so we prefer arrays, which on nightly are really full featured!

    // we can create an array from a function
    let c: [i32; LEN] = core::array::from_fn(|n| n as i32 + 1);
    rprintln!("{:?}", c);

    // And arrays support a few iterator like functions like map
    let d: [i32; LEN] = A.map(|v| v + 1);
    rprintln!("{:?}", d);

    // but can't yet collect iterators into an array yet https://github.com/rust-lang/rfcs/pull/2990
    // so well use a crate with similar functionality called heapless
    let e: heapless::Vec<i32, LEN> = A.iter().zip(B.iter()).map(|(a, b)| a + b).collect();

    rprintln!("{:?}", e);

    loop {
        cortex_m::asm::nop();
    }
}
