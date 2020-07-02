//! This project is used for generating a tone signal from L-Tek audio board.
//! Here, we have a digital input signal as single tone sine wave. This signal
//! is represented with x array in main.c file. The frequency of this signal is
//! Pi/8 rad/sec. It is is fed to L-Tek audio board repeatedly.
//!
//! Requires cargo embed `cargo install cargo-embed`
//!
//! `cargo embed --example 5_6_tone_generation`
//!
//! I2S from https://github.com/maxekman/stm32f407g-disc/commit/662a337e86ce2b381661f5c8b7a8fa93ceed8481

#![no_std]
#![no_main]

use stm32f4xx_hal as hal;

use crate::hal::{delay::Delay, i2c::*, i2s::*, interrupt, prelude::*, stm32};
use core::f32::consts::PI;
use cortex_m::interrupt::free;
use cortex_m_rt::entry;
use micromath::F32Ext;
use panic_rtt as _;
use typenum::Unsigned;

//hrmm. .larger or custom types? https://github.com/paholg/typenum/issues/131
type N = heapless::consts::U8000;

const AUDIO_FREQ: u32 = 8000;

const OMEGA: f32 = 2.0 * PI * 500.0;
const OMEGA_S: f32 = 2.0 * PI * 8000.0;

#[entry]
fn main() -> ! {
    let mut dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // Set up the system clock.
    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz()) //discovery board has 8 MHz crystal for HSE
        .sysclk(168.mhz())
        // .pclk1(24.mhz())
        // .pclk2(24.mhz())
        // .pclk1(42.mhz())
        .plli2sclk(AUDIO_FREQ.hz())
        .freeze();

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();
    let gpiod = dp.GPIOD.split();

    let mut delay = Delay::new(cp.SYST, clocks);

    // Setup I2C1 using PB6 and PB9 pins at 100kHz bitrate.
    let scl = gpiob.pb6.into_alternate_af4().set_open_drain();
    let sda = gpiob.pb9.into_alternate_af4().set_open_drain();
    let i2c1 = I2c::i2c1(dp.I2C1, (scl, sda), 100.khz(), clocks);

    // CS43L22 reset pin.
    let reset = gpiod.pd4.into_push_pull_output();

    let _cs43l22 = cs43l22::CS43L22::new(i2c1, reset, &mut delay, 128).unwrap();

    // Set PC7 into AF6 to output the MCLK for I2S3.
    let _mck = gpioc.pc7.into_alternate_af6();

    // Setup I2S3 for 48kHz audio.
    let ck = gpioc.pc10.into_alternate_af6();
    let ws = gpioa.pa4.into_alternate_af6();
    let sd = gpioc.pc12.into_alternate_af6();

    // AudioInit(FS_8000_HZ, AUDIO_INPUT_MIC, IO_METHOD_INTR);
    let mut i2s3 = I2s::i2s3(dp.SPI3, (ck, ws, sd, NoSdExt), AUDIO_FREQ.hz(), clocks);

    let sin = (0..N::to_usize())
        .map(|n| {
            // Obtaining discrete-time frequency component w
            let w = 2.0 * PI * OMEGA / OMEGA_S;
            // arm_scale_f32(x, 8192, x, N);
            // arm_offset_f32(x, 8192, x, N);
            (w * n as f32).sin() * 8192.0 + 8192.0
        })
        .collect::<heapless::Vec<u16, N>>();

    // i2s api is blocking currently
    loop {
        for s in sin.iter().cloned() {
            let _ = i2s3.send(s); //u16 left
            let _ = i2s3.send(s); //u16 right
        }
    }

    // let mut s: u16 = 0;
    // let mut y: u16 = 100;
    // loop {
    //     // Send both left and right word.
    //     i2s3.send(s).unwrap();
    //     i2s3.send(s).unwrap();

    //     // Sawtooth with incrementing pitch each cycle.
    //     if s >= (65535 - y) {
    //         s = 0;
    //         y += 1;
    //         if y > 400 {
    //             y = 100
    //         }
    //     }
    //     s += y;
    // }
}
