//! This project is used for explaining the DFT operation using standard math
//! functions. Here, we have a digital input signal as the sum of two sinusoids
//! with different frequencies. The complex form of this signal is represented
//! with s_complex array, the frequency component of this signal is found by the
//! DFT function. Real and imaginary parts of the obtained DFT are represented
//! with XR and XI arrays. The magnitude of DFT is kept in the Mag array.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over no no_std without alloc.
//!
//! `cargo run --example 4_1_dft_calculations`

use core::f32::consts::PI;
use textplots::{Chart, Plot, Shape};

const N: usize = 256;

const W1: f32 = core::f32::consts::PI / 128.0;
const W2: f32 = core::f32::consts::PI / 4.0;
// const W2: f32 = core::f32::consts::PI / 5.0;

fn main() {
    // Complex sum of sinusoidal signals
    let s1 = (0..N).map(|val| (W1 * val as f32).sin());
    let s2 = (0..N).map(|val| (W2 * val as f32).sin());
    let s = s1.zip(s2).map(|(ess1, ess2)| ess1 + ess2);

    // map it to real, leave im blank well fill in with dft
    let dtfsecoef = s.map(|f| Complex32 { re: f, im: 0.0 });

    let dft = dft(dtfsecoef).collect::<heapless::Vec<Complex32, N>>();

    let re = dft
        .iter()
        .map(|complex| complex.re)
        .collect::<heapless::Vec<f32, N>>();
    display("re", re.iter().cloned());

    let im = dft
        .iter()
        .map(|complex| complex.im)
        .collect::<heapless::Vec<f32, N>>();
    display("im", im.iter().cloned());

    //Magnitude calculation
    let mag = dft
        .iter()
        .map(|complex| (complex.re * complex.re + complex.im * complex.im).sqrt())
        .collect::<heapless::Vec<f32, N>>();
    display("mag", mag.iter().cloned());
}

struct Complex32 {
    re: f32,
    im: f32,
}

fn dft<I: Iterator<Item = Complex32> + Clone>(input: I) -> impl Iterator<Item = Complex32> {
    let size = N as f32;
    (0..N).map(move |k| {
        let k = k as f32;
        let mut sum_re = 0.0;
        let mut sum_im = 0.0;
        for (n, complex) in input.clone().enumerate() {
            let n = n as f32;
            sum_re += complex.re * (2.0 * PI * k * n / size).cos()
                + complex.im * (2.0 * PI * k * n / size).sin();
            sum_im += -complex.im * (2.0 * PI * k * n / size).cos()
                + complex.re * (2.0 * PI * k * n / size).sin();
        }

        Complex32 {
            re: sum_re,
            im: -sum_im,
        }
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
    let display = input
        .enumerate()
        .map(|(idx, y)| (idx as f32, y))
        .collect::<Vec<(f32, f32)>>();
    Chart::new(120, 60, 0.0, N as f32)
        .lineplot(Shape::Lines(&display[..]))
        .display();
}
