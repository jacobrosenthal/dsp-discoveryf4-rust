//! Led Blinky Roulette example using the DWT peripheral for timing.
//!
//! This project is used for creating five different basic digital signals: unit pulse, unit step, unit ramp, exponential and sinusoidal. These signals are represented with d1, u1, r, e1 and s arrays in main.rs file.
//!
//! Open this project in Keil, debug it and run the code as explained in Lab 0 of the lab manual. Then you can export these five arrays using Export.ini file as explained in Section 0.4.3 of the lab manual. This file is already available in the project folder.  
//!
//! Requires cargo flash
//!
//! `cargo install cargo-flash`
//!
//! `cargo flash --example roulette --release --chip STM32F407VGTx --protocol swd`

#![no_std]
#![no_main]

use stm32f4xx_hal as hal;

use crate::hal::{dwt::ClockDuration, dwt::DwtExt, prelude::*, stm32};
pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
use cortex_m_rt::entry;
use panic_halt as _;

use core::f32::consts::PI;
use micromath::F32Ext;

const N: usize = 16;

const W1: f32 = core::f32::consts::PI / 128f32;
const W2: f32 = core::f32::consts::PI / 4f32;

fn DFT(x: &[f32], XR: &mut [f32], XI: &mut [f32]) {
    let size = XR.len();
    (0..size).for_each(|k| {
        let mut sumR = 0.0;
        let mut sumI = 0.0;

        let ss = size as f32;

        (0..size).for_each(|n| {
            let nn = n as f32;
            let k = k as f32;

            sumR += x[2 * n + 0] * (2f32 * PI * k * nn / ss).cos()
                + x[2 * n + 1] * (2f32 * PI * k * nn / ss).sin();
            sumI += -x[2 * n + 1] * (2f32 * PI * k * nn / ss).cos()
                + x[2 * n + 0] * (2f32 * PI * k * nn / ss).sin();
        });
        XR[k] = sumR;
        XI[k] = -sumI;
    });
}

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

    let mut itm = cp.ITM;

    iprintln!(&mut itm.stim[0], "Hello, world!");

    // Create a delay abstraction based on DWT cycle counter
    let dwt = cp.DWT.constrain(cp.DCB, clocks);

    //Sinusoidal signals
    let mut s1 = [0f32; N];
    let mut s2 = [0f32; N];
    let mut s = [0f32; N];

    (0..N).for_each(|n| {
        s1[n] = (W1 * n as f32).sin();
        s2[n] = (W2 * n as f32).sin();
        s[n] = s1[n] + s2[n];
    });

    //Complex sum of sinusoidal signals
    let mut s_complex = [0f32; 2 * N];

    (0..N).for_each(|n| {
        s_complex[2 * n] = s[n];
        s_complex[2 * n + 1] = 0f32;
    });

    let mut XR = [0f32; N];
    let mut XI = [0f32; N];
    let mut Mag = [0f32; N];

    //set a breakpoint and inspect
    let time: ClockDuration = dwt.measure(|| {
        DFT(&s_complex, &mut XR, &mut XI);

        //Magnitude calculation
        Mag.iter_mut()
            .enumerate()
            .for_each(|(n, mag_ref)| *mag_ref = (XR[n].powf(2f32) + XI[n].powf(2f32)).sqrt());
    });

    loop {}
}
