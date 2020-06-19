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
use micromath::F32Ext;
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
use heapless::consts::U16;
use microfft::{complex::cfft_16, Complex32};
use typenum::Unsigned;

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

    //square signal
    let square = (0..U16::to_usize()).map(|idx| if idx < U16::to_usize() / 2 { 1.0 } else { 0.0 });

    //map it to real, leave im blank well fill in with cfft
    let mut dtfsecoef = square
        .clone()
        .map(|f| Complex32 { re: f, im: 0.0 })
        .collect::<heapless::Vec<Complex32, U16>>();

    // Coefficient calculation with CFFT function
    // arm_cfft_f32 uses a forward transform with enables bit reversal of output
    // well use microfft uses an in place Radix-2 FFT, for some reasons returns itself we dont need
    let _ = cfft_16(&mut dtfsecoef[..]);

    let time: ClockDuration = dwt.measure(|| {
        let _y_real =
            dtfse::<U16, _>(dtfsecoef.iter().cloned(), 15).collect::<heapless::Vec<f32, U16>>();
    });
    dbgprint!("ticks: {:?}", time.as_ticks());

    loop {}
}

fn dtfse<N: Unsigned, I: Iterator<Item = Complex32> + Clone>(
    coeff: I,
    k_var: usize,
) -> impl Iterator<Item = f32> {
    let size = N::to_usize() as f32;
    (0..N::to_usize()).map(move |n| {
        coeff
            .clone()
            .take(k_var + 1)
            .enumerate()
            .map(|(k, complex)| {
                let a = (complex.re * complex.re + complex.im * complex.im).sqrt();
                let p = complex.im.atan2(complex.re);
                a * ((2.0 * PI * k as f32 * n as f32 / size) + p).cos() / size
            })
            .sum::<f32>()
    })
}
