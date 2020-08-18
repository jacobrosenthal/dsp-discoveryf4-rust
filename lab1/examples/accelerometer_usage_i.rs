//! Basic BSP accelerometer functions and how they are used are given below.
//!
//! Requires `cargo install cargo-flash`
//! `cargo flash --example accelerometer_usage_i --release`

#![no_std]
#![no_main]

use panic_halt as _;
use stm32f4xx_hal as hal;

use accelerometer::RawAccelerometer;
use hal::{prelude::*, spi, stm32};
use lis302dl::Lis302Dl;

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

    let gpioa = dp.GPIOA.split();
    let gpioe = dp.GPIOE.split();
    let gpiod = dp.GPIOD.split();

    let sck = gpioa.pa5.into_alternate_af5().internal_pull_up(false);
    let miso = gpioa.pa6.into_alternate_af5().internal_pull_up(false);
    let mosi = gpioa.pa7.into_alternate_af5().internal_pull_up(false);

    let spi_mode = spi::Mode {
        polarity: spi::Polarity::IdleLow,
        phase: spi::Phase::CaptureOnFirstTransition,
    };

    let spi = spi::Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        spi_mode,
        10.mhz().into(),
        clocks,
    );

    let mut chip_select = gpioe.pe3.into_push_pull_output();
    chip_select.set_high().ok();

    let mut lis302dl = Lis302Dl::new(spi, chip_select, Default::default());

    let mut top = gpiod.pd12.into_push_pull_output();
    let mut left = gpiod.pd13.into_push_pull_output();
    let mut right = gpiod.pd14.into_push_pull_output();
    let mut bottom = gpiod.pd15.into_push_pull_output();

    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    loop {
        let dat = lis302dl.accel_raw().unwrap();

        //not entirely sure this represents the example exactly..
        if dat.z > 50 {
            if dat.x < 10 && dat.x > -10 && dat.y < 10 && dat.y > -10 {
                top.set_high().ok();
                left.set_high().ok();
                right.set_high().ok();
                bottom.set_high().ok();
            } else {
                top.set_low().ok();
                left.set_low().ok();
                right.set_low().ok();
                bottom.set_low().ok();
            }
        }
        delay.delay_ms(10u8);
    }
}
