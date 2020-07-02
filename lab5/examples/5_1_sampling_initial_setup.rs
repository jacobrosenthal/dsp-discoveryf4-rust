//! The PA1 pin is used for the ADC channel. The ADC1 module is configured for
//! 12 bit single conversion from Channel 1. The timer module triggers the ADC
//! every 1/16000 sec. Also the ADC interrupt is enabled such that when the
//! conversion ends, an interrupt is generated.
//!
//! Requires cargo embed `cargo install cargo-embed`
//!
//! `cargo embed --example 5_1_sampling_initial_setup`

#![no_std]
#![no_main]

use stm32f4xx_hal as hal;

use crate::hal::{
    adc::{config::AdcConfig, config::SampleTime, Adc},
    prelude::*,
    stm32,
};

use cortex_m_rt::entry;
use jlink_rtt;
use panic_rtt as _;
use typenum::Unsigned;

macro_rules! dbgprint {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut out = $crate::jlink_rtt::Output::new();
            writeln!(out, $($arg)*).ok();
        }
    };
}

type N = heapless::consts::U100;

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

    // let mut delay = Delay::new(cp.SYST, clocks);
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    let gpioa = dp.GPIOA.split();
    let pa1 = gpioa.pa1.into_analog();

    let mut adc = Adc::adc1(dp.ADC1, true, AdcConfig::default());

    // doing blocking reads instead of interrupt driven
    let x = (0..N::to_usize())
        .map(|_| {
            delay.delay_us(62u16); //0.0000625 s is  62.5us? 16.khz()
            adc.convert(&pa1, SampleTime::Cycles_84)
        })
        .collect::<heapless::Vec<u16, N>>();

    dbgprint!("x: {:?}", &x[..]);

    loop {}
}