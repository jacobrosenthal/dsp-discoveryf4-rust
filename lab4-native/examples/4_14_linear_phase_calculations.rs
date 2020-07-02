//! This project is used for explaining the linear phase property of digital
//! filters. Here we have a low-pass filter represented by h array. First its
//! FFT is calculated using the arm_cfft_f32 function. Then the magnitude and
//! phase of the FFT are stored in Mag and Phase arrays.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over no no_std without alloc.
//!
//! `cargo run --example 4_14_linear_phase_calculations`

use textplots::{Chart, Plot, Shape};

use core::f32::consts::PI;
use itertools::Itertools;
use microfft::{complex::cfft_64, Complex32};
use typenum::Unsigned;

type N = heapless::consts::U64;

fn main() {
    // Complex impulse response of filter
    let mut dtfsecoef = H
        .iter()
        .cloned()
        .map(|h| Complex32 { re: h, im: 0.0 })
        .collect::<heapless::Vec<Complex32, N>>();

    let _ = cfft_64(&mut dtfsecoef[..]);

    // Magnitude calculation
    let mag = dtfsecoef
        .iter()
        .map(|complex| (complex.re * complex.re + complex.im * complex.im).sqrt())
        .collect::<heapless::Vec<f32, N>>();
    display::<N, _>("mag", mag.iter().cloned());

    let phase = dtfsecoef
        .iter()
        .cloned()
        .map(|complex| complex.re.atan2(complex.im));

    // not sure why yet, but this is how they display in the matlab file
    let phase_graph = phase
        .clone()
        .enumerate()
        .map(|(i, phase)| if i < 33 { phase } else { phase - PI });

    display::<N, _>("phase", phase_graph.clone());
}

// Points isn't a great representation as you can lose the line in the graph,
// however while Lines occasionally looks good it also can be terrible.
// Continuous requires to be in a fn pointer closure which cant capture any
// external data so not useful without lots of code duplication.
fn display<N, I>(name: &str, input: I)
where
    N: Unsigned,
    I: Iterator<Item = f32> + core::clone::Clone + std::fmt::Debug,
{
    println!("{:?}: {:?}", name, input.clone().format(", "));
    let display = input
        .enumerate()
        .map(|(n, y)| (n as f32, y))
        .collect::<Vec<(f32, f32)>>();
    Chart::new(120, 60, 0.0, N::to_usize() as f32)
        .lineplot(Shape::Points(&display[..]))
        .display();
}

// linear_phase_FIR_coefficients
#[rustfmt::skip]
static H: &'static [f32] = &[
    0.002110571833, 0.003037402174,  0.00401057303, 0.005026416387, 0.006080887746,
    0.007169586606, 0.008287782781, 0.009430442937,  0.01059226226,  0.01176769473,
    0.01295099314,  0.01413624361,  0.01531740464,   0.0164883472,  0.01764290221,
    0.018774895,   0.0198781956,  0.02094675414,  0.02197465487,  0.02295614779,
    0.02388569713,  0.02475801855,  0.02556811832,   0.0263113305,  0.02698334865,
    0.02758026123,  0.02809858322,   0.0285352692,  0.02888775431,   0.0291539561,
    0.02933230437,  0.02942174487,  0.02942174487,  0.02933230437,   0.0291539561,
    0.02888775431,   0.0285352692,  0.02809858322,  0.02758026123,  0.02698334865,
    0.0263113305,  0.02556811832,  0.02475801855,  0.02388569713,  0.02295614779,
    0.02197465487,  0.02094675414,   0.0198781956,    0.018774895,  0.01764290221,
    0.0164883472,  0.01531740464,  0.01413624361,  0.01295099314,  0.01176769473,
    0.01059226226, 0.009430442937, 0.008287782781, 0.007169586606, 0.006080887746,
    0.005026416387,  0.00401057303, 0.003037402174, 0.002110571833
];
