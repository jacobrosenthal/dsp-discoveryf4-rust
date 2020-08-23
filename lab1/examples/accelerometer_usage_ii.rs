//! This project is used for acquiring the accelerometer data as a digital
//! signal.
//!
//! Requires `cargo install probe-run`
//! `cargo run --release --example accelerometer_usage_ii`

#![no_std]
#![no_main]

use panic_break as _;
use stm32f4xx_hal as hal;

use hal::{prelude::*, spi, stm32};
use lis3dsh::{accelerometer::RawAccelerometer, Lis3dsh};
use rtt_target::{rprintln, rtt_init_print};

const N: usize = 1000;

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!(BlockIfFull, 128);

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

    let mut buffer = [0i16; N];
    buffer.iter_mut().for_each(|buffer_ref| {
        while !lis3dsh.is_data_ready().unwrap() {}
        *buffer_ref = lis3dsh.accel_raw().unwrap()[0];
    });

    rprintln!("{:?}", buffer);

    // signal to probe-run to exit
    loop {
        cortex_m::asm::bkpt()
    }
}
