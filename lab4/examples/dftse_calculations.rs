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

use crate::hal::{dwt::ClockDuration, dwt::DwtExt, prelude::*, stm32};
pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
use cortex_m_rt::entry;
use microfft::{complex::cfft_16, Complex32};
use panic_halt as _;

use core::f32::consts::PI;
use micromath::F32Ext;

const N: usize = 256;
const K: usize = 1;

const W1: f32 = core::f32::consts::PI / 128f32;
const W2: f32 = core::f32::consts::PI / 4f32;

fn DTFSE(X: &mut [f32], xc: &[f32], Kx: usize) {
    let mut P = 0f32;
    let mut A = 0f32;

    let size = X.len();

    X.iter_mut().enumerate().for_each(|(n, x_ref)| {
        let mut sumR = 0f32;

        (0..Kx).for_each(|k| {
            let kk = k as usize;
            A = (xc[2 * kk] * xc[2 * kk] + xc[2 * kk + 1] * xc[2 * kk + 1]).sqrt();
            P = (xc[2 * kk + 1]).atan2(xc[2 * kk]);
            sumR += A * ((2f32 * PI * k as f32 * n as f32 / size as f32) + P).cos() / size as f32;
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

    let mut itm = cp.ITM;

    iprintln!(&mut itm.stim[0], "Hello, world!");

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

    let mut DTFSEcoef = s_complex.clone();

    // //Coefficient calculation with CFFT function
    // arm_cfft_f32(&arm_cfft_sR_f32_len16, DTFSEcoef, 0, 1);//forward transform(not inverse), enables bit reversal of output(With it set to 0 the bins are all mixed up)

    let result = cfft_16(&mut DTFSEcoef).unwrap();

    //set a breakpoint and inspect
    let time: ClockDuration = dwt.measure(|| {
        DTFSE(&mut y_real, &DTFSEcoef, K);
    });

    loop {}
}
