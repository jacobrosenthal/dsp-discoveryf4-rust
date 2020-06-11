//! This project is used for creating eight different frame-based digital
//! systems.
//!
//! Requires cargo embed `cargo install cargo-embed`
//!
//! `cargo embed --example 2_11_heapless_frame_based_systems`

#![no_std]
#![no_main]

use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, stm32};
use cortex_m_rt::entry;
use jlink_rtt;
use panic_rtt as _;

mod utils;

use heapless::consts::U10;
const N: usize = 10;

macro_rules! dbgprint {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut out = $crate::jlink_rtt::NonBlockingOutput::new();
            writeln!(out, $($arg)*).ok();
        }
    };
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

    //y[n] = b x[n]
    let digital_system1 = utils::unit_step(0..N)
        .map(|unit| 2.2 * unit)
        .collect::<heapless::Vec<f32, U10>>();
    dbgprint!("digital_system1: {:?}", &digital_system1);

    //y[n] = x1[n] + x2[n]
    let digital_system2 = utils::unit_step(0..N)
        .zip(utils::sinusoidal(0..N))
        .map(|(unit, s)| unit + s)
        .collect::<heapless::Vec<f32, U10>>();
    dbgprint!("digital_system2: {:?}", &digital_system2);

    //y[n] = x^2[n]
    //-0.5881598.powf(2f32) overflowing on micromath 489298620000.0, use multiplication
    let digital_system3 = utils::sinusoidal(0..N)
        .map(|s| s * s)
        .collect::<heapless::Vec<f32, U10>>();
    dbgprint!("digital_system3: {:?}", &digital_system3);

    //y[n] = b0 x[n] + b1 x[n-1]
    //random backwards access.. so no iterators unless ... todo
    let mut digital_system4 = [0f32; N];
    let sinusoidal = utils::sinusoidal(0..N).collect::<heapless::Vec<f32, U10>>();
    digital_system4
        .iter_mut()
        .enumerate()
        .for_each(|(idx, out)| {
            if idx == 0 {
                *out = 2.2 * sinusoidal[idx]
            } else {
                *out = 2.2 * sinusoidal[idx] + -1.1 * sinusoidal[idx - 1]
            }
        });
    dbgprint!("digital_system4: {:?}", &digital_system4);

    //y[n] = b0 x[n] + b1 x[n-1] + a1 y[n-1]
    //random backwards access.. so no iterators unless ... todo
    let mut digital_system5 = [0f32; N];
    let sinusoidal = utils::sinusoidal(0..N).collect::<heapless::Vec<f32, U10>>();
    //cant enumerate over output either since random access
    for idx in 0..digital_system5.len() {
        if idx == 0 {
            digital_system5[idx] = 2.2 * sinusoidal[idx];
        } else {
            digital_system5[idx] =
                2.2 * sinusoidal[idx] + -1.1 * sinusoidal[idx - 1] + 0.7 * digital_system5[idx - 1];
        }
    }
    dbgprint!("digital_system5: {:?}", &digital_system5);

    //y[n] = b0 x[n+1] + b1 x[n]
    //digital_system6 in c version has oob array access, should be if (n+1 < size) so y6[9] undefined
    let unit_pulse = utils::unit_step(0..N).collect::<heapless::Vec<f32, U10>>();
    let digital_system6 = unit_pulse
        .windows(2)
        .map(|unit_window| 2.2 * unit_window[1] + -1.1 * unit_window[0])
        .collect::<heapless::Vec<f32, U10>>();
    dbgprint!("digital_system6: {:?}", &digital_system6);

    //y[n] = b0 x[n] + a1 y[n-1]
    //random backwards access.. so no iterators unless ... todo
    let mut digital_system7 = [0f32; N];
    let unit_pulse = utils::unit_pulse(0..N).collect::<heapless::Vec<f32, U10>>();
    for idx in 0..digital_system5.len() {
        if idx == 0 {
            digital_system7[idx] = 1.0 * unit_pulse[idx]
        } else {
            digital_system7[idx] = 1.0 * unit_pulse[idx] + 2.0 * digital_system7[idx - 1]
        }
    }

    dbgprint!("digital_system7: {:?}", &digital_system7);

    //y[n] = n x[n]
    let digital_system8 = utils::sinusoidal(0..N)
        .enumerate()
        .map(|(idx, s)| idx as f32 * s)
        .collect::<heapless::Vec<f32, U10>>();
    dbgprint!("digital_system8: {:?}", &digital_system8);

    loop {}
}
