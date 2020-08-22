//! Basic BSP accelerometer functions and how they are used are given below.
//!
//! Requires `cargo install cargo-flash`
//! `cargo flash --example accelerometer_usage_i --release`

#![no_std]
#![no_main]

use panic_halt as _;
use stm32f4xx_hal as hal;

use hal::{prelude::*, spi, stm32};
use lis3dsh::Lis3dsh;

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

    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

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

    let chip_select = gpioe.pe3.into_push_pull_output();
    let mut lis3dsh = Lis3dsh::new_spi(spi, chip_select);
    lis3dsh.init(&mut delay).unwrap();
    assert_eq!(lis3dsh.who_am_i().unwrap(), lis3dsh::EXPECTED_WHO_AM_I);

    let mut top = gpiod.pd12.into_push_pull_output();
    let mut left = gpiod.pd13.into_push_pull_output();
    let mut right = gpiod.pd14.into_push_pull_output();
    let mut bottom = gpiod.pd15.into_push_pull_output();

    loop {
        while !lis3dsh.is_data_ready().unwrap() {}
        let dat = lis3dsh.read_data().unwrap();

        //not entirely sure this represents the example exactly..
        if dat[2] > 50 {
            if dat[0] < 10 && dat[0] > -10 && dat[1] < 10 && dat[1] > -10 {
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
