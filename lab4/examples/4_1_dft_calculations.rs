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
pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
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
use itertools::Itertools;
use micromath::F32Ext;

const N: usize = 256;

const W1: f32 = core::f32::consts::PI / 128.0;
const W2: f32 = core::f32::consts::PI / 4.0;

fn DFT(x: &[f32], XR: &mut [f32], XI: &mut [f32]) {
    debug_assert!(XR.len() == XI.len());
    debug_assert!(x.len() == 2 * XI.len());

    let size = XR.len();

    XR.iter_mut()
        .zip(XI.iter_mut())
        .enumerate()
        .for_each(|(idx, (xr_ref, xi_ref))| {
            let mut sumR = 0.0;
            let mut sumI = 0.0;

            x.iter().tuples().enumerate().for_each(|(n, (x0, x1))| {
                let something = 2.0 * PI * idx as f32 * n as f32 / size as f32;

                sumR += x0 * something.cos() + x1 * something.sin();
                sumI += -x1 * something.cos() + x0 * something.sin();
            });
            *xr_ref = sumR;
            *xi_ref = -sumI;
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
        s_complex[2 * n + 1] = 0.0;
    });

    let mut XR = [0f32; N];
    let mut XI = [0f32; N];
    let mut Mag = [0f32; N];

    let time: ClockDuration = dwt.measure(|| {
        DFT(&s_complex, &mut XR, &mut XI);

        //Magnitude calculation
        Mag.iter_mut()
            .zip(XR.iter())
            .zip(XI.iter())
            .for_each(|((mag_ref, xr), xi)| *mag_ref = (xr * xr + xi * xi).sqrt());
    });
    dbgprint!("dft ticks: {:?}", time.as_ticks());
    dbgprint!("XR: {:?}", &XR[..]);
    dbgprint!("XI: {:?}", &XI[..]);

    loop {}
}
