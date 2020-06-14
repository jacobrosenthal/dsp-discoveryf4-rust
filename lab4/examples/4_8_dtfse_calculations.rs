//! This project is used for explaining the DTFSE operation. Here, we have a
//! periodic square signal. The complex form of this signal is represented with
//! s_complex array. DTFSE coefficients are calculated, then, the signal is
//! approximated with the DTFSE function. This function returns its output in
//! real form because original signal has only real parts in this example. The
//! result is kept in the y_real array.
//!
//! Requires cargo embed `cargo install cargo-embed`
//!
//! `cargo embed --example 4_8_dtfse_calculations`

#![no_std]
#![no_main]

use stm32f4xx_hal as hal;

use crate::hal::{dwt::ClockDuration, dwt::DwtExt, prelude::*, stm32};
use cortex_m_rt::entry;
use jlink_rtt;
use microfft::{complex::cfft_512, Complex32};
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

use core::f32::consts::PI;
use micromath::F32Ext;

const N: usize = 16;
const K: usize = 1;

const W1: f32 = core::f32::consts::PI / 128.0;
const W2: f32 = core::f32::consts::PI / 4.0;

fn DTFSE(X: &mut [f32], xc: &[f32], Kx: usize) {
    let size = X.len();

    X.iter_mut().enumerate().for_each(|(n, x_ref)| {
        let mut sumR = 0.0;

        (0..Kx).for_each(|k| {
            let kk = k as usize;
            let A = (xc[2 * kk] * xc[2 * kk] + xc[2 * kk + 1] * xc[2 * kk + 1]).sqrt();
            let P = (xc[2 * kk + 1]).atan2(xc[2 * kk]);
            sumR += A * ((2.0 * PI * k as f32 * n as f32 / size as f32) + P).cos() / size as f32;
        });

        *x_ref = sumR;
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

    // Create a delay abstraction based on DWT cycle counter
    let dwt = cp.DWT.constrain(cp.DCB, clocks);

    let mut s_real = [0f32; N];
    let mut s_imag = [0f32; N];
    let mut y_real = [0f32; N];
    let mut s_complex = [0f32; 2 * N];

    //square signal
    (0..N).for_each(|n| {
        if n < N / 2 {
            s_real[n] = 1.0;
        } else {
            s_real[n] = 0.0;
        }

        s_imag[n] = 0.0;
        s_complex[2 * n + 0] = s_real[n];
        s_complex[2 * n + 1] = s_imag[n];
    });

    // Coefficient calculation with CFFT function
    // let mut DTFSEcoef = s_complex.clone();
    // let mut DTFSEcoef = [Complex32::default(); 512];
    // forward transform(not inverse), enables bit reversal of output(With it set to 0 the bins are all mixed up)
    // arm_cfft_f32(&arm_cfft_sR_f32_len16, DTFSEcoef, 0, 1);
    // let result = cfft_512(&mut DTFSEcoef);

    let time: ClockDuration = dwt.measure(|| {
        DTFSE(&mut y_real, &s_complex[..], K);
        dbgprint!("y_real: {:?}", &y_real[..]);
    });
    dbgprint!("ticks: {:?}", time.as_ticks());

    loop {}
}
