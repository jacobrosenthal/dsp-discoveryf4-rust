//! This project is used for explaining the DFT operation using standard math
//! functions. Here, we have a digital input signal as the sum of two sinusoids
//! with different frequencies. The complex form of this signal is represented
//! with s_complex array, the frequency component of this signal is found by the
//! DFT function. Real and imaginary parts of the obtained DFT are represented
//! with XR and XI arrays. The magnitude of DFT is kept in the Mag array.
//!
//! Requires cargo embed
//! `cargo install cargo-embed`
//!
//! `cargo embed --example 4_1_dft_calculations`

#![no_std]
#![no_main]

use stm32f4xx_hal as hal;

use crate::hal::{dwt::ClockDuration, dwt::DwtExt, prelude::*, stm32};
use cortex_m_rt::entry;
use jlink_rtt;
use panic_rtt as _;

macro_rules! dbgprint {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut out = $crate::jlink_rtt::Output::new();
            writeln!(out, $($arg)*).ok();
        }
    };
}

use core::f32::consts::PI;
use heapless::consts::U256;
use itertools::Itertools;
use micromath::F32Ext;

const N: usize = 256;
const W1: f32 = core::f32::consts::PI / 128.0;
const W2: f32 = core::f32::consts::PI / 4.0;

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

    // Create a delay abstraction based on DWT cycle counter
    let dwt = cp.DWT.constrain(cp.DCB, clocks);

    //Sinusoidal signals
    let s1 = (0..N)
        .map(|val| (W1 * val as f32).sin())
        .collect::<heapless::Vec<f32, U256>>();
    let s2 = (0..N)
        .map(|val| (W2 * val as f32).sin())
        .collect::<heapless::Vec<f32, U256>>();
    let s = s1
        .iter()
        .zip(s2.iter())
        .map(|(ess1, ess2)| ess1 + ess2)
        .collect::<heapless::Vec<f32, U256>>();

    //Complex sum of sinusoidal signals
    //todo not sure how to get an iter of a tuple of (s,0) so I can just collect here...
    let mut s_complex = [0f32; 2 * N];
    s.iter()
        .zip(s_complex.iter_mut().tuples())
        .for_each(|(s, (s0, s1))| {
            *s0 = *s;
            *s1 = 0.0;
        });

    let time: ClockDuration = dwt.measure(|| {
        //todo dont like passing as a slice...
        let dft = DFT::new(&s_complex[..]);
        //Magnitude calculation
        let mag = dft.mag_iter().collect::<heapless::Vec<f32, U256>>();
    });
    dbgprint!("dft ticks: {:?}", time.as_ticks());

    loop {}
}

struct DFT {
    XR: heapless::Vec<f32, U256>,
    XI: heapless::Vec<f32, U256>,
}

impl<'a> DFT {
    // todo dont like taking this as a slice.. but not sure on lifetimes as i
    // needs x to be copy or clone or something because its going to create an
    // iter from it many times..
    fn new(x: &[f32]) -> Self {
        //todo, building each seperately optimizes far worse I think
        Self {
            XR: (0..N)
                .map(|idx| {
                    x.iter()
                        .tuples()
                        .enumerate()
                        .map(|(n, (x0, x1))| {
                            let something = 2.0 * PI * idx as f32 * n as f32 / N as f32;

                            x0 * something.cos() + x1 * something.sin()
                        })
                        .sum::<f32>()
                })
                .collect::<heapless::Vec<f32, U256>>(),

            XI: (0..256)
                .map(|idx| {
                    -x.iter()
                        .tuples()
                        .enumerate()
                        .map(|(n, (x0, x1))| {
                            let something = 2.0 * PI * idx as f32 * n as f32 / N as f32;

                            -x1 * something.cos() + x0 * something.sin()
                        })
                        .sum::<f32>()
                })
                .collect::<heapless::Vec<f32, U256>>(),
        }
    }

    fn mag_iter(&'a self) -> impl Iterator<Item = f32> + 'a {
        self.XR
            .iter()
            .zip(self.XI.iter())
            .map(|(xr, xi)| (xr * xr + xi * xi).sqrt())
    }
}
