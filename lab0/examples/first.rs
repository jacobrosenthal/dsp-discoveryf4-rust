//! Demo iterators.
//!
//! Requires `cargo install cargo-embed`
//!
//! cargo-embed builds and uploads your code and maintains a two way connection
//! which in combination with panic-rtt-target and rtt-target lets you send and
//! receive debug data over long lasting sessions.
//!
//! `cargo embed --example roulette --release`

#![no_std]
#![no_main]

use panic_rtt_target as _;
use stm32f4xx_hal as hal;

use hal::{prelude::*, stm32};
use heapless::{consts::*, Vec};
use rtt_target::{rprintln, rtt_init_print};

static A: &[i32] = &[1, 2, 3, 4, 5];
static B: &[i32] = &[1, 2, 3, 4, 5];

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

    //can't collect into an array, so use a heapless (static) vec
    let c = A
        .iter()
        .zip(B.iter())
        .map(|(a, b)| a + b)
        .collect::<Vec<_, U5>>();

    rprintln!("{:?}", c);

    loop {}
}
