//! This project is used for explaining the DFT operation using standard math
//! functions. Here, we have a digital input signal as the sum of two sinusoids
//! with different frequencies. The complex form of this signal is represented
//! with s_complex array, the frequency component of this signal is found by the
//! DFT function. Real and imaginary parts of the obtained DFT are represented
//! with XR and XI arrays. The magnitude of DFT is kept in the Mag array.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over to no_std without alloc.
//!
//! `cargo run --example 4_1_dft_calculations`

#![feature(array_from_fn)]

use core::f32::consts::PI;
use lab4::{display, Shape};

const N: usize = 256;

const W1: f32 = core::f32::consts::PI / 128.0;
const W2: f32 = core::f32::consts::PI / 4.0;
// const W2: f32 = core::f32::consts::PI / 5.0;

fn main() {
    // Complex sum of sinusoidal signals
    let dtfsecoef: [Complex32; N] = core::array::from_fn(|n| Complex32 {
        re: (W1 * n as f32).sin() + (W2 * n as f32).sin(),
        im: 0.0,
    });

    let dft: heapless::Vec<Complex32, N> = dft(dtfsecoef.iter().cloned()).collect();

    let re: heapless::Vec<f32, N> = dft.iter().map(|complex| complex.re).collect();
    display("re", Shape::Line, re.iter().cloned());

    let im: heapless::Vec<f32, N> = dft.iter().map(|complex| complex.im).collect();
    display("im", Shape::Line, im.iter().cloned());

    //Magnitude calculation
    let mag = dft
        .iter()
        .map(|complex| (complex.re * complex.re + complex.im * complex.im).sqrt());

    display("mag", Shape::Line, mag);
}

fn dft<I: Iterator<Item = Complex32> + Clone>(input: I) -> impl Iterator<Item = Complex32> {
    let size = N as f32;
    (0..N).map(move |k| {
        input
            .clone()
            .enumerate()
            .fold((0f32, 0f32), |(mut sum_re, mut sum_im), (n, complex)| {
                let n = n as f32;
                sum_re += complex.re * (2.0 * PI * k as f32 * n / size).cos()
                    + complex.im * (2.0 * PI * k as f32 * n / size).sin();
                sum_im += -complex.im * (2.0 * PI * k as f32 * n / size).cos()
                    + complex.re * (2.0 * PI * k as f32 * n / size).sin();

                (sum_re, sum_im)
            })
            .into()
    })
}

#[repr(C)]
#[derive(Clone, Debug)]
struct Complex32 {
    re: f32,
    im: f32,
}

impl From<(f32, f32)> for Complex32 {
    fn from(incoming: (f32, f32)) -> Self {
        Complex32 {
            re: incoming.0,
            im: incoming.1,
        }
    }
}
