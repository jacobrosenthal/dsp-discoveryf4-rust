//! Led Blinky Roulette example using the DWT peripheral for timing.
//!
//! Requires `cargo install cargo-flash`
//!
//! cargo-flash builds and uploads your code to run standalone. Its a more
//! traditional method of uploading firmware which doesn't maintain any
//! communication link for debug information.
//!
//! `cargo flash --example roulette --release`

#![no_std]
#![no_main]

use panic_halt as _;
use stm32f4xx_hal as hal;

use hal::{dwt::DwtExt, prelude::*, stm32};

#[cortex_m_rt::entry]
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

    let gpiod = dp.GPIOD.split();
    let mut led1 = gpiod.pd12.into_push_pull_output();
    let mut led2 = gpiod.pd13.into_push_pull_output();
    let mut led3 = gpiod.pd14.into_push_pull_output();
    let mut led4 = gpiod.pd15.into_push_pull_output();

    loop {
        led1.set_high().unwrap();
        led2.set_low().unwrap();
        led3.set_low().unwrap();
        led4.set_low().unwrap();
        delay.delay_ms(333_u32);

        led1.set_low().unwrap();
        led2.set_high().unwrap();
        led3.set_low().unwrap();
        led4.set_low().unwrap();
        delay.delay_ms(333_u32);

        led1.set_low().unwrap();
        led2.set_low().unwrap();
        led3.set_high().unwrap();
        led4.set_low().unwrap();
        delay.delay_ms(333_u32);

        led1.set_low().unwrap();
        led2.set_low().unwrap();
        led3.set_low().unwrap();
        led4.set_high().unwrap();
        delay.delay_ms(333_u32);
    }
}
