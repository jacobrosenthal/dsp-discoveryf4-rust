//! This project is used for creating eight different digital signals by applying different operations on basic digital signals.
//!
//! Requires cargo embed
//! `cargo install cargo-embed`
//!
//! `cargo embed --release --example operating_on_signals`

#![no_std]
#![no_main]

use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, stm32};
use cortex_m_rt::entry;
use jlink_rtt;
use panic_rtt as _;

macro_rules! dbgprint {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut out = $crate::jlink_rtt::NonBlockingOutput::new();
            writeln!(out, $($arg)*).ok();
        }
    };
}

use micromath::F32Ext;

const N: usize = 10;
const A: f32 = 0.8;
const W0: f32 = core::f32::consts::PI / 5f32;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let _cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // Set up the system clock.
    let rcc = dp.RCC.constrain();

    let _clocks = rcc
        .cfgr
        .use_hse(8.mhz()) //discovery board has 8 MHz crystal for HSE
        .sysclk(168.mhz())
        .freeze();

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
    dbgprint!("x1: {:?}", &x1[..]);

    //elevated sinusoidal signal
    let mut x2 = [0f32; N];
    x2.iter_mut()
        .zip(&sinusoidal)
        .for_each(|(val, ess)| *val = ess + 1f32);
    dbgprint!("x2: {:?}", &x2[..]);

    //negated unit step signal
    let mut x3 = [0i32; N];
    x3.iter_mut()
        .zip(&unit_step)
        .for_each(|(val, uu)| *val = -uu);
    dbgprint!("x3: {:?}", &x3[..]);

    //applying all operations on the sinusoidal signal
    let mut x4 = [0f32; N];
    x4.iter_mut()
        .skip(2)
        .zip(&sinusoidal)
        .for_each(|(val, ess)| *val = 3f32 * *ess - 2f32);
    dbgprint!("x4: {:?}", &x4[..]);

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
    dbgprint!("x5: {:?}", &x5[..]);

    // //multiplying the exponential signal with the unit step signal
    let mut x6 = [0f32; N];
    x6.iter_mut()
        .zip(&exponential)
        .zip(&unit_step)
        .for_each(|((val, e), u)| *val = e * *u as f32);
    dbgprint!("x6: {:?}", &x6[..]);

    // //multiplying the exponential signal with the sinusoidal signal
    let mut x7 = [0f32; N];
    x7.iter_mut()
        .zip(&exponential)
        .zip(&sinusoidal)
        .for_each(|((val, e), s)| *val = e * s);
    dbgprint!("x7: {:?}", &x7[..]);

    //multiplying the exponential signal with the window signal
    let mut x8 = [0f32; N];
    x8.iter_mut()
        .zip(&exponential)
        .zip(&x5)
        .for_each(|((val, e), x)| *val = e * *x as f32);
    dbgprint!("x8: {:?}", &x8[..]);

    loop {}
}
