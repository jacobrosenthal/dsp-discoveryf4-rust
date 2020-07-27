//! Demo iterators.
//!
//! With cargo flash
//! `cargo install cargo-flash`
//!
//! `cargo flash --example first --release`

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use heapless::{consts::*, Vec};
use panic_halt as _;
use stm32f4xx_hal::{prelude::*, stm32};

static A: &[i32] = &[1, 2, 3, 4, 5];
static B: &[i32] = &[1, 2, 3, 4, 5];

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    // Set up the system clock.
    let rcc = dp.RCC.constrain();

    let _ = rcc
        .cfgr
        .use_hse(8.mhz()) //discovery board has 8 MHz crystal for HSE
        .sysclk(168.mhz())
        .freeze();

    let gpiod = dp.GPIOD.split();
    let mut green = gpiod.pd12.into_push_pull_output();

    //can't collect into an array, so use a heapless (static) vec
    let c = A
        .iter()
        .zip(B.iter())
        .map(|(a, b)| a + b)
        .collect::<Vec<_, U5>>();

    if c == [2, 4, 6, 8, 10] {
        let _ = green.set_high();
    }

    loop {}
}
