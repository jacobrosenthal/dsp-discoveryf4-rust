//! This project is used for creating eight different frame-based digital
//! systems.
//!
//! Requires cargo embed `cargo install cargo-embed`
//!
//! `cargo embed --release --example frame_based_systems`

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
const W0: f32 = core::f32::consts::PI / 5.0;

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

//-0.5881598.powf(2f32) overflowing on micromath 489298620000.0, use multiplication
fn digital_system3(input: &[f32], output: &mut [f32]) {
    output
        .iter_mut()
        .zip(input)
        .for_each(|(out_ref, inny)| *out_ref = inny * inny)
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
    let _cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // Set up the system clock.
    let rcc = dp.RCC.constrain();

    let _clocks = rcc
        .cfgr
        .use_hse(8.mhz()) //discovery board has 8 MHz crystal for HSE
        .sysclk(168.mhz())
        .freeze();

    //unit pulse signal
    let mut unit_pulse = [0f32; N];
    unit_pulse.iter_mut().enumerate().for_each(|(idx, val)| {
        if idx == 0 {
            *val = 1.0;
        } else {
            *val = 0.0;
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
    dbgprint!("sinusoidal: {:?}", &sinusoidal);

    //y[n] = b x[n]
    let mut y1 = [0f32; N];
    digital_system1(2.2, &unit_step, &mut y1);
    dbgprint!("digital_system1: {:?}", &y1);

    //y[n] = x1[n] + x2[n]
    let mut y2 = [0f32; N];
    digital_system2(&unit_step, &sinusoidal, &mut y2);
    dbgprint!("digital_system2: {:?}", &y2);

    //y[n] = x^2[n]
    let mut y3 = [0f32; N];
    digital_system3(&sinusoidal, &mut y3);
    dbgprint!("digital_system3: {:?}", &y3);

    //y[n] = b0 x[n] + b1 x[n-1]
    let mut y4 = [0f32; N];
    digital_system4(&[2.2, -1.1], &sinusoidal, &mut y4);
    dbgprint!("digital_system4: {:?}", &y4);

    //y[n] = b0 x[n] + b1 x[n-1] + a1 y[n-1]
    let mut y5 = [0f32; N];
    digital_system5(&[2.2, -1.1], 0.7, &sinusoidal, &mut y5);
    dbgprint!("digital_system5: {:?}", &y5);

    //y[n] = b0 x[n+1] + b1 x[n]
    let mut y6 = [0f32; N];
    digital_system6(&[2.2, -1.1], &unit_step, &mut y6);
    dbgprint!("digital_system6: {:?}", &y6);

    //y[n] = b0 x[n] + a1 y[n-1]
    let mut y7 = [0f32; N];
    digital_system7(1.0, 2.0, &unit_pulse, &mut y7);
    dbgprint!("digital_system7: {:?}", &y7);

    //y[n] = n x[n]
    let mut y8 = [0f32; N];
    digital_system8(&sinusoidal, &mut y8);
    dbgprint!("digital_system8: {:?}", &y8);

    loop {}
}
