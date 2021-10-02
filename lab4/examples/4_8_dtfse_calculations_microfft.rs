//! This project is used for explaining the DTFSE operation. Here, we have a
//! periodic square signal. The complex form of this signal is represented with
//! s_complex array. DTFSE coefficients are calculated, then, the signal is
//! approximated with the DTFSE function. This function returns its output in
//! real form because original signal has only real parts in this example. The
//! result is kept in the y_real array.
//!
//! Requires `cargo install probe-run`
//! `cargo run --release --example 4_8_dtfse_calculations_microfft`

#![no_std]
#![no_main]
#![feature(array_from_fn)]

use panic_probe as _;
use stm32f4xx_hal as hal;

use core::f32::consts::PI;
use hal::{dwt::ClockDuration, dwt::DwtExt, prelude::*, stm32};
use microfft::Complex32;
use micromath::F32Ext;
use rtt_target::{rprintln, rtt_init_print};

use microfft::complex::cfft_16 as cfft;
const N: usize = 16;

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!(BlockIfFull, 128);

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

    // square signal
    let square: [f32; N] = core::array::from_fn(|n| if n < N / 2 { 1.0 } else { 0.0 });

    // map it to real, leave im blank well fill in with cfft
    let mut dtfsecoef = square.map(|f| Complex32 { re: f, im: 0.0 });

    // Coefficient calculation with CFFT function
    // well use microfft uses an in place Radix-2 FFT
    let _ = cfft(&mut dtfsecoef);

    let time: ClockDuration = dwt.measure(|| {
        let _y_real: heapless::Vec<_, N> = dtfse(dtfsecoef.iter().cloned(), 15).collect();
    });
    rprintln!("ticks: {:?}", time.as_ticks());

    // signal to probe-run to exit
    loop {
        cortex_m::asm::bkpt()
    }
}

fn dtfse<I: Iterator<Item = Complex32> + Clone>(
    coeff: I,
    k_var: usize,
) -> impl Iterator<Item = f32> {
    let size = N as f32;
    (0..N).map(move |n| {
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
