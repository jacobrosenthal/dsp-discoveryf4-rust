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
use panic_halt as _;

use micromath::F32Ext;

const N: usize = 10;
const A: f32 = 0.8;
const W0: f32 = core::f32::consts::PI / 5f32;

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
    let mut unit_pulse = [0; N];
    unit_pulse.iter_mut().enumerate().for_each(|(idx, val)| {
        if idx == 0 {
            *val = 1;
        } else {
            *val = 0;
        }
    });

    //unit step signal
    let unit_step = [1; N];

    //exponential signal
    let mut exponential = [0f32; N];
    exponential
        .iter_mut()
        .enumerate()
        .for_each(|(idx, val)| *val = A.powf(idx as f32));

    //sinusoidal signal
    let mut sinusoidal = [0f32; N];
    sinusoidal
        .iter_mut()
        .enumerate()
        .for_each(|(idx, val)| *val = (W0 * idx as f32).sin());

    //shifted unit pulse signal
    let mut x1 = [0i32; N];
    x1.iter_mut()
        .skip(4)
        .zip(&unit_pulse)
        .for_each(|(val, dee)| *val = *dee);

    //elevated sinusoidal signal
    let mut x2 = [0f32; N];
    x2.iter_mut()
        .zip(&sinusoidal)
        .for_each(|(val, ess)| *val = ess + 1f32);

    //negated unit step signal
    let mut x3 = [0i32; N];
    x3.iter_mut()
        .zip(&unit_step)
        .for_each(|(val, uu)| *val = -uu);

    //applying all operations on the sinusoidal signal
    let mut x4 = [0f32; N];
    x4.iter_mut()
        .skip(2)
        .zip(&sinusoidal)
        .for_each(|(val, ess)| *val = 3f32 * *ess - 2f32);

    //subtracting two unit step signals
    let mut x5 = [0i32; N];
    x5.iter_mut()
        .zip(&unit_step)
        .zip(unit_step.iter().skip(4))
        .enumerate()
        .for_each(|(idx, ((val, u1), udelay))| {
            if idx < 4 {
                *val = *u1;
            } else {
                *val = u1 - udelay;
            }
        });

    // //multiplying the exponential signal with the unit step signal
    let mut x6 = [0f32; N];
    x6.iter_mut()
        .zip(&exponential)
        .zip(&unit_step)
        .for_each(|((val, e), u)| *val = e * *u as f32);

    // //multiplying the exponential signal with the sinusoidal signal
    let mut x7 = [0f32; N];
    x7.iter_mut()
        .zip(&exponential)
        .zip(&sinusoidal)
        .for_each(|((val, e), s)| *val = e * s);

    //multiplying the exponential signal with the window signal
    let mut x8 = [0f32; N];
    x8.iter_mut()
        .zip(&exponential)
        .zip(&x5)
        .for_each(|((val, e), x)| *val = e * *x as f32);

    loop {}
}
