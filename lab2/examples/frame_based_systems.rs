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
use panic_halt as _;

use micromath::F32Ext;

const N: usize = 10;
const W0: f32 = core::f32::consts::PI / 5f32;

fn digital_system1(b: f32, input: &[f32], output: &mut [f32]) {
    output
        .iter_mut()
        .zip(input)
        .for_each(|(out_ref, inny)| *out_ref = b * *inny)
}

fn digital_system2(input1: &[f32], input2: &[f32], output: &mut [f32]) {
    output
        .iter_mut()
        .zip(input1.iter().zip(input2))
        .for_each(|(out_ref, (inny1, inny2))| *out_ref = inny1 + inny2)
}

fn digital_system3(input: &[f32], output: &mut [f32]) {
    output
        .iter_mut()
        .zip(input)
        .for_each(|(out_ref, inny)| *out_ref = inny.powf(2f32))
}

fn digital_system4(b: &[f32], input: &[f32], output: &mut [f32]) {
    //random backwards access.. so no iterators unless ... todo
    for idx in 0..output.len() {
        if idx == 0 {
            output[idx] = b[0] * input[idx];
        } else {
            output[idx] = b[0] * input[idx] + b[1] * input[idx - 1];
        }
    }
}

fn digital_system5(b: &[f32], a: f32, input: &[f32], output: &mut [f32]) {
    //random backwards access.. so no iterators unless ... todo
    for idx in 0..output.len() {
        if idx == 0 {
            output[idx] = b[0] * input[idx];
        } else {
            output[idx] = b[0] * input[idx] + b[1] * input[idx - 1] + a * output[idx - 1];
        }
    }
}

fn digital_system6(b: &[f32], input: &[f32], output: &mut [f32]) {
    output
        .iter_mut()
        .zip(input.windows(2))
        .for_each(|(out_ref, inny)| *out_ref = b[0] * inny[1] + b[1] * inny[0])
}

fn digital_system7(b: f32, a: f32, input: &[f32], output: &mut [f32]) {
    //random backwards access.. so no iterators unless ... todo
    for idx in 0..output.len() {
        if idx == 0 {
            output[idx] = b * input[idx];
        } else {
            output[idx] = b * input[idx] + a * output[idx - 1];
        }
    }
}

fn digital_system8(input: &[f32], output: &mut [f32]) {
    output
        .iter_mut()
        .zip(input)
        .enumerate()
        .for_each(|(idx, (out_ref, inny))| *out_ref = idx as f32 * inny)
}

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
    let mut unit_pulse = [0f32; N];
    unit_pulse.iter_mut().enumerate().for_each(|(idx, val)| {
        if idx == 0 {
            *val = 1f32;
        } else {
            *val = 0f32;
        }
    });

    //unit step signal
    let unit_step = [1f32; N];

    //sinusoidal signal
    let mut sinusoidal = [0f32; N];
    sinusoidal
        .iter_mut()
        .enumerate()
        .for_each(|(idx, val)| *val = (W0 * idx as f32).sin());

    //y[n] = b x[n]
    let mut y1 = [0f32; N];
    digital_system1(2.2, &unit_step, &mut y1);

    //y[n] = x1[n] + x2[n]
    let mut y2 = [0f32; N];
    digital_system2(&unit_step, &sinusoidal, &mut y2);

    //y[n] = x^2[n]
    let mut y3 = [0f32; N];
    digital_system3(&sinusoidal, &mut y3);

    //y[n] = b0 x[n] + b1 x[n-1]
    let mut y4 = [0f32; N];
    digital_system4(&[2.2, -1.1], &sinusoidal, &mut y4);

    //y[n] = b0 x[n] + b1 x[n-1] + a1 y[n-1]
    let mut y5 = [0f32; N];
    digital_system5(&[2.2, -1.1], 0.7, &sinusoidal, &mut y5);

    //y[n] = b0 x[n+1] + b1 x[n]
    let mut y6 = [0f32; N];
    digital_system6(&[2.2, -1.1], &unit_step, &mut y6);

    //y[n] = b0 x[n] + a1 y[n-1]
    let mut y7 = [0f32; N];
    digital_system7(1.0, 2.0, &unit_pulse, &mut y7);

    //y[n] = n x[n]
    let mut y8 = [0f32; N];
    digital_system8(&sinusoidal, &mut y8);

    loop {}
}
