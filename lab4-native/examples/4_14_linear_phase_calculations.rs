//! This project is used for explaining the linear phase property of digital
//! filters. Here we have a low-pass filter represented by h array. First its
//! FFT is calculated using the arm_cfft_f32 function. Then the magnitude and
//! phase of the FFT are stored in Mag and Phase arrays.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over to no_std without alloc.
//!
//! `cargo run --example 4_14_linear_phase_calculations`

use core::f32::consts::PI;
use lab4::{display, Shape};
use microfft::Complex32;

use microfft::complex::cfft_64 as cfft;

fn main() {
    // Complex impulse response of filter
    let mut dtfsecoef = H.map(|h| Complex32 { re: h, im: 0.0 });

    // Coefficient calculation with CFFT function
    // well use microfft uses an in place Radix-2 FFT
    let _ = cfft(&mut dtfsecoef);

    // Magnitude calculation
    let mag = dtfsecoef.map(|complex| (complex.re * complex.re + complex.im * complex.im).sqrt());

    display("mag", Shape::Line, mag.iter().cloned());

    let phase = dtfsecoef
        .iter()
        .cloned()
        .map(|complex| complex.re.atan2(complex.im));

    // not sure why yet, but this is how they display in the matlab file
    let phase_graph = phase
        .clone()
        .enumerate()
        .map(|(i, phase)| if i < 33 { phase } else { phase - PI });

    display("phase", Shape::Line, phase_graph.clone());
}

// linear_phase_FIR_coefficients
#[rustfmt::skip]
static H: [f32; 64] = [
    0.002_110_571_8, 0.003_037_402_2,  0.004_010_573, 0.005_026_416_4, 0.006_080_887_7,
    0.007_169_586_6, 0.008_287_783, 0.009_430_443,  0.010_592_262,  0.011_767_695,
    0.012_950_993,  0.014_136_244,  0.015_317_405,   0.016_488_347,  0.017_642_902,
    0.018774895,   0.019_878_196,  0.020_946_754,  0.021_974_655,  0.022_956_148,
    0.023_885_697,  0.024_758_019,  0.025_568_118,   0.026_311_33,  0.026_983_349,
    0.027_580_261,  0.028_098_583,   0.028_535_27,  0.028_887_754,   0.029_153_956,
    0.029_332_304,  0.029_421_745,  0.029_421_745,  0.029_332_304,   0.029_153_956,
    0.028_887_754,   0.028_535_27,  0.028_098_583,  0.027_580_261,  0.026_983_349,
    0.026_311_33,  0.025_568_118,  0.024_758_019,  0.023_885_697,  0.022_956_148,
    0.021_974_655,  0.020_946_754,   0.019_878_196,    0.018774895,  0.017_642_902,
    0.016_488_347,  0.015_317_405,  0.014_136_244,  0.012_950_993,  0.011_767_695,
    0.010_592_262, 0.009_430_443, 0.008_287_783, 0.007_169_586_6, 0.006_080_887_7,
    0.005_026_416_4,  0.004_010_573, 0.003_037_402_2, 0.002_110_571_8
];
