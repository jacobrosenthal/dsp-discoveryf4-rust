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
use itertools::Itertools;

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
    let y1 = utils::unit_step(0..N)
        .map(|u| 2.2 * u)
        .collect::<heapless::Vec<f32, U10>>();
    dbgprint!("digital_system1: {:?}", &y1);

    //y[n] = x1[n] + x2[n]
    let y2 = utils::sinusoidal(0..N)
        .zip(utils::unit_step(0..N))
        .map(|(inny1, inny2)| inny1 + inny2)
        .collect::<heapless::Vec<f32, U10>>();
    dbgprint!("digital_system2: {:?}", &y2);

    //y[n] = x^2[n]
    let y3 = utils::sinusoidal(0..N)
        .map(|inny| inny * inny)
        .collect::<heapless::Vec<f32, U10>>();
    dbgprint!("digital_system3: {:?}", &y3);

    //y[n] = b0 x[n] + b1 x[n-1]
    let y4 = DigitalSystem4::new(utils::sinusoidal(0..N)).collect::<heapless::Vec<f32, U10>>();
    dbgprint!("digital_system4: {:?}", &y4);

    //y[n] = b0 x[n] + b1 x[n-1] + a1 y[n-1]
    let y5 = DigitalSystem5::new(utils::sinusoidal(0..N)).collect::<heapless::Vec<f32, U10>>();
    dbgprint!("digital_system5: {:?}", &y5);

    //y[n] = b0 x[n+1] + b1 x[n]
    // digital_system6 in c version has oob array access, should be if (n+1 < size) so y6[9] undefined
    let y6 = utils::unit_step(0..N)
        .tuple_windows()
        .map(|(u0, u1)| 2.2 * u1 + -1.1 * u0)
        .collect::<heapless::Vec<f32, U10>>();
    dbgprint!("digital_system6: {:?}", &y6);

    //y[n] = b0 x[n] + a1 y[n-1]
    let y7 = DigitalSystem7::new(utils::unit_pulse(0..N)).collect::<heapless::Vec<f32, U10>>();
    dbgprint!("digital_system7: {:?}", &y7);

    //y[n] = n x[n]
    let y8 = utils::sinusoidal(0..N)
        .enumerate()
        .map(|(idx, inny)| idx as f32 * inny)
        .collect::<heapless::Vec<f32, U10>>();

    dbgprint!("digital_system8: {:?}", &y8);

    loop {}
}

struct DigitalSystem4<I>
where
    I: Iterator<Item = f32>,
{
    last: Option<f32>,
    iter: I,
}
impl<I> DigitalSystem4<I>
where
    I: Iterator<Item = f32>,
{
    fn new(iter: I) -> Self {
        Self {
            last: None,
            iter: iter,
        }
    }
}

impl<I> Iterator for DigitalSystem4<I>
where
    I: Iterator<Item = f32>,
{
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if let Some(val) = self.iter.next() {
            let abc = if let Some(last) = self.last {
                2.2 * val + -1.1 * last
            } else {
                2.2 * val
            };

            self.last = Some(val);
            Some(abc)
        } else {
            None
        }
    }
}

struct DigitalSystem5<I>
where
    I: Iterator<Item = f32>,
{
    last_in: Option<f32>,
    last_out: Option<f32>,
    iter: I,
}

impl<I> DigitalSystem5<I>
where
    I: Iterator<Item = f32>,
{
    fn new(iter: I) -> Self {
        Self {
            last_in: None,
            last_out: None,
            iter: iter,
        }
    }
}

impl<I> Iterator for DigitalSystem5<I>
where
    I: Iterator<Item = f32>,
{
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if let Some(val) = self.iter.next() {
            let abc = if let (Some(last_in), Some(last_out)) = (self.last_in, self.last_out) {
                2.2 * val + -1.1 * last_in + -1.1 * last_out
            } else {
                2.2 * val
            };

            self.last_in = Some(val);
            Some(abc)
        } else {
            None
        }
    }
}

struct DigitalSystem7<I>
where
    I: Iterator<Item = f32>,
{
    last_out: Option<f32>,
    iter: I,
}

impl<I> DigitalSystem7<I>
where
    I: Iterator<Item = f32>,
{
    fn new(iter: I) -> Self {
        Self {
            last_out: None,
            iter: iter,
        }
    }
}
impl<I> Iterator for DigitalSystem7<I>
where
    I: Iterator<Item = f32>,
{
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if let Some(val) = self.iter.next() {
            self.last_out = if let Some(last_out) = self.last_out {
                Some(1.0 * val + 2.0 * last_out)
            } else {
                Some(1.0 * val)
            };

            self.last_out
        } else {
            None
        }
    }
}
