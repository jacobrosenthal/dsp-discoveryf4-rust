//! This project is used for explaining filtering in frequency domain. Here, we
//! have a digital input signal as the sum of two sinusoids with different
//! frequencies. The complex form of this signal is represented with s_complex
//! array in main.c file. Also we have a digital filter represented with h array
//! given in FIR_lpf_coefficients.h file.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over to no_std without alloc.
//!
//! `cargo run --example 4_10_stft_calculations`

use core::f32::consts::PI;
use microfft::Complex32;
use textplots::{Chart, Plot, Shape};

use microfft::complex::cfft_512 as cfft;
const N: usize = 512;

const W1: f32 = core::f32::consts::PI / 128.0;
const W2: f32 = core::f32::consts::PI / 4.0;

fn main() {
    // Complex sum of sinusoidal signals
    let s1 = (0..N).map(|val| (W1 * val as f32).sin());
    let s2 = (0..N).map(|val| (W2 * val as f32).sin());
    let s = s1.zip(s2).map(|(ess1, ess2)| ess1 + ess2);

    let mut s_complex: heapless::Vec<Complex32, N> =
        s.clone().map(|f| Complex32 { re: f, im: 0.0 }).collect();

    // Complex impulse response of filter
    let mut df_complex: heapless::Vec<Complex32, N> = H
        .iter()
        .cloned()
        .map(|f| Complex32 { re: f, im: 0.0 })
        .chain(core::iter::repeat(Complex32 { re: 0.0, im: 0.0 }))
        //fill rest with zeros up to N
        .take(N)
        .collect();

    // SAFETY microfft now only accepts arrays instead of slices to avoid runtime errors
    // Thats not great for us. However we can cheat since our slice into an array because
    // "The layout of a slice [T] of length N is the same as that of a [T; N] array."
    // https://rust-lang.github.io/unsafe-code-guidelines/layout/arrays-and-slices.html
    // this goes away when something like heapless vec is in standard library
    // https://github.com/rust-lang/rfcs/pull/2990
    unsafe {
        let ptr = &mut *(df_complex.as_mut_ptr() as *mut [Complex32; N]);

        // Finding the FFT of the filter
        let _ = cfft(ptr);
    }

    // SAFETY same as above
    unsafe {
        let ptr = &mut *(s_complex.as_mut_ptr() as *mut [Complex32; N]);

        // Finding the FFT of the input signal
        let _ = cfft(ptr);
    }

    // Filtering in the frequency domain
    let y_complex = s_complex
        .iter()
        .zip(df_complex.iter())
        //multiply complex
        .map(|(s, df)| Complex32 {
            re: s.re * df.re - s.im * df.im,
            im: s.re * df.im + s.im * df.re,
        });

    // Finding the complex result in time domain
    // supposed to be inverse transform but microfft doesnt have it
    // Could patch it in. inverse DFT is the same as the DFT, but with the
    // opposite sign in the exponent and a 1/N factor, any FFT algorithm can
    // easily be adapted for it.
    // just dtfse approx instead for now
    let y_freq: heapless::Vec<f32, N> = dtfse(y_complex.clone(), 15).collect();
    display("freq", y_freq.iter().cloned());

    //y_time via convolution_sum developed in 2.14 to compare
    let y_time: heapless::Vec<f32, N> = convolution_sum(s).collect();
    display("time", y_time.iter().cloned());
}

static H: &[f32] = &[
    0.002044, 0.007806, 0.014554, 0.020018, 0.024374, 0.027780, 0.030370, 0.032264, 0.033568,
    0.034372, 0.034757, 0.034791, 0.034534, 0.034040, 0.033353, 0.032511, 0.031549, 0.030496,
    0.029375, 0.028207, 0.027010, 0.025800, 0.024587, 0.023383, 0.022195, 0.021031, 0.019896,
    0.018795, 0.017730, 0.016703, 0.015718, 0.014774, 0.013872, 0.013013, 0.012196, 0.011420,
    0.010684, 0.009989, 0.009331, 0.008711, 0.008127, 0.007577, 0.007061, 0.006575, 0.006120,
    0.005693, 0.005294, 0.004920, 0.004570, 0.004244, 0.003939, 0.003655, 0.003389, 0.003142,
    0.002912, 0.002698, 0.002499, 0.002313, 0.002141, 0.001981, 0.001833, 0.001695, 0.001567,
    0.001448,
];

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

pub fn convolution_sum<I>(x: I) -> impl Iterator<Item = f32> + Clone
where
    I: Iterator<Item = f32>
        + core::iter::ExactSizeIterator
        + core::iter::DoubleEndedIterator
        + Clone,
{
    (0..x.len()).map(move |y_n| {
        x.clone()
            .take(y_n + 1)
            .rev()
            .zip(H.iter())
            .map(|(exx, h)| h * exx)
            .sum()
    })
}

// Points isn't a great representation as you can lose the line in the graph,
// however while Lines occasionally looks good it also can be terrible.
// Continuous requires to be in a fn pointer closure which cant capture any
// external data so not useful without lots of code duplication.
fn display<I>(name: &str, input: I)
where
    I: Iterator<Item = f32> + core::clone::Clone + std::fmt::Debug,
{
    println!("{:?}:", name);
    let display: Vec<(f32, f32)> = input.enumerate().map(|(n, y)| (n as f32, y)).collect();
    Chart::new(120, 60, 0.0, N as f32)
        .lineplot(&Shape::Lines(&display))
        .display();
}
