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
    let mut d1: [i32; N] = [0; N];
    for (n, val) in d1.iter_mut().enumerate() {
        if n == 0 {
            *val = 1;
        } else {
            *val = 0;
        }
    }

    //unit step signal
    let mut u1: [i32; N] = [0; N];
    for val in u1.iter_mut() {
        *val = 1;
    }

    //unit ramp signal
    let mut r: [i32; N] = [0; N];
    for (n, val) in r.iter_mut().enumerate() {
        //todo as could panic right?
        *val = n as i32;
    }

    //exponential signal
    let mut e1: [f64; N] = [0f64; N];
    for (n, val) in e1.iter_mut().enumerate() {
        //todo as could panic right?
        *val = pow(A, n as f64);
    }

    //sinusoidal signal
    let mut s: [f64; N] = [0f64; N];
    for (n, val) in s.iter_mut().enumerate() {
        //todo as could panic right?
        *val = sin(W0 * (n as f64));
    }

    loop {}
}
