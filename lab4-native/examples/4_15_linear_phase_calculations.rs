//! This project is used for explaining the linear phase property of digital
//! filters. Here we have a low-pass filter represented by h array. First its
//! FFT is calculated using the arm_cfft_f32 function. Then the magnitude and
//! phase of the FFT are stored in Mag and Phase arrays.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over to no_std without alloc.
//!
//! `cargo run --example 4_15_linear_phase_calculations`

use itertools::Itertools;
use microfft::Complex32;
use textplots::{Chart, Plot, Shape};

use microfft::complex::cfft_64 as cfft;
const N: usize = 64;

fn main() {
    // Complex impulse response of filter
    let mut dtfsecoef = H
        .iter()
        .cloned()
        .map(|h| Complex32 { re: h, im: 0.0 })
        .collect::<heapless::Vec<Complex32, N>>();

    let _ = cfft(&mut dtfsecoef);

    // Magnitude calculation
    let mag = dtfsecoef
        .iter()
        .map(|complex| (complex.re * complex.re + complex.im * complex.im).sqrt())
        .collect::<heapless::Vec<f32, N>>();
    display("mag", mag.iter().cloned());

    let phase = dtfsecoef
        .iter()
        .cloned()
        .map(|complex| complex.re.atan2(complex.im));

    display("phase", phase.clone());
}

// Points isn't a great representation as you can lose the line in the graph,
// however while Lines occasionally looks good it also can be terrible.
// Continuous requires to be in a fn pointer closure which cant capture any
// external data so not useful without lots of code duplication.
fn display<I>(name: &str, input: I)
where
    I: Iterator<Item = f32> + core::clone::Clone + std::fmt::Debug,
{
    println!("{:?}: {:.4?}", name, input.clone().format(", "));
    let display = input
        .enumerate()
        .map(|(n, y)| (n as f32, y))
        .collect::<Vec<(f32, f32)>>();
    Chart::new(120, 60, 0.0, N as f32)
        .lineplot(&Shape::Lines(&display))
        .display();
}

// FIR_lpf_coefficients for 4_15
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
