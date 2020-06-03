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

//todo as could panic right?

#![no_std]
#![no_main]

use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, stm32};
pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
use cortex_m_rt::entry;
use libm::{pow, sin};
use panic_halt as _;

const N: usize = 10;
const A: f64 = 0.8;
const W0: f64 = core::f64::consts::PI / 5f64;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // Set up the system clock.
    let rcc = dp.RCC.constrain();

    let _clocks = rcc
        .cfgr
        .use_hse(8.mhz()) //discovery board has 8 MHz crystal for HSE
        .sysclk(168.mhz())
        .freeze();

    let mut itm = cp.ITM;

    iprintln!(&mut itm.stim[0], "Hello, world!");

    //unit pulse signal
    let mut unit_pulse = [0i32; N];
    for (n, val) in unit_pulse.iter_mut().enumerate() {
        if n == 0 {
            *val = 1;
        } else {
            *val = 0;
        }
    }

    //unit step signal
    let mut unit_step = [0i32; N];
    for val in unit_step.iter_mut() {
        *val = 1;
    }

    //exponential signal
    let mut exponential = [0f64; N];
    for (n, val) in exponential.iter_mut().enumerate() {
        *val = pow(A, n as f64);
    }

    //sinusoidal signal
    let mut sinusoidal = [0f64; N];
    for (n, val) in sinusoidal.iter_mut().enumerate() {
        *val = sin(W0 * (n as f64));
    }

    //shifted unit pulse signal
    let mut x1 = [0i32; N];
    for (val, dee) in x1.iter_mut().skip(4).zip(&unit_pulse) {
        *val = *dee;
    }

    //elevated sinusoidal signal
    let mut x2 = [0f64; N];
    for (val, ess) in x2.iter_mut().zip(&sinusoidal) {
        *val = ess + 1f64;
    }

    //negated unit step signal
    let mut x3 = [0i32; N];
    for (val, uu) in x3.iter_mut().zip(&unit_step) {
        *val = -uu;
    }

    //applying all operations on the sinusoidal signal
    let mut x4 = [0f64; N];
    for (val, ess) in x4.iter_mut().skip(2).zip(&sinusoidal) {
        *val = 3f64 * *ess - 2f64;
    }

    //subtracting two unit step signals
    let mut x5 = [0i32; N];
    for (n, ((val, u1), udelay)) in x5
        .iter_mut()
        .zip(&unit_step)
        .zip(unit_step.iter().skip(4))
        .enumerate()
    {
        if n < 4 {
            *val = *u1;
        } else {
            *val = u1 - udelay;
        }
    }

    // //multiplying the exponential signal with the unit step signal
    let mut x6 = [0f64; N];
    for ((val, e), u) in x6.iter_mut().zip(&exponential).zip(&unit_step) {
        *val = e * *u as f64;
    }

    // //multiplying the exponential signal with the sinusoidal signal
    let mut x7 = [0f64; N];
    for ((val, e), s) in x7.iter_mut().zip(&exponential).zip(&sinusoidal) {
        *val = e * s;
    }

    //multiplying the exponential signal with the window signal
    let mut x8 = [0f64; N];
    for ((val, e), x) in x8.iter_mut().zip(&exponential).zip(&x5) {
        *val = e * *x as f64;
    }

    loop {}
}
