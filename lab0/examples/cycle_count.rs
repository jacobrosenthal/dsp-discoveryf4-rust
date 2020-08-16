//! This project is used to measure the code execution in terms of clock cycles.
//!
//! Requires `cargo install probe-run`
//!
//! probe-run builds, uploads, and runs your code on device and in combination
//! with rtt-target and panic-break prints debug and panic information to your
//! console. Its used for short running sessions like seeing the results of a
//! calculation or a measurement, a panic message or backtrace of an error right
//! on your command line. It exits when it detects a breakpoint.
//!
//!`cargo run --release --example cycle_count`

#![no_std]
#![no_main]

use panic_break as _;
use stm32f4xx_hal as hal;

use hal::{dwt::ClockDuration, dwt::DwtExt, prelude::*, stm32};
use rtt_target::{rprintln, rtt_init_print};

#[cortex_m_rt::entry]
fn main() -> ! {
    // setup the rtt machinery for printing
    rtt_init_print!(BlockIfFull);

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
    rprintln!("ticks: {:?}", time.as_ticks());

    // signal to probe-run to exit
    loop {
        cortex_m::asm::bkpt()
    }
}
