//! This project is used for explaining the DTFSE operation. Here, we have a
//! periodic square signal. The complex form of this signal is represented with
//! s_complex array. DTFSE coefficients are calculated, then, the signal is
//! approximated with the DTFSE function. This function returns its output in
//! real form because original signal has only real parts in this example. The
//! result is kept in the y_real array.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over to no_std without alloc.
//!
//! `cargo run --example 4_9`

use core::f32::consts::PI;
use itertools::Itertools;
use microfft::Complex32;
use textplots::{Chart, Plot, Shape};

const N: usize = 16;
use microfft::complex::cfft_16 as cfft;

const TRIANGLE_AMPLITUDE: f32 = 1.5;
const TRIANGLE_PERIOD: usize = 16;

fn main() {
    // Collecting to turn the Cycle into a clean iterator for our naive display fn
    let triangle = (0..TRIANGLE_PERIOD)
        .map(|n| {
            let period = TRIANGLE_PERIOD as f32;

            if n < (TRIANGLE_PERIOD / 2) {
                (2.0 * TRIANGLE_AMPLITUDE / (period / 2.0)) * n as f32 - TRIANGLE_AMPLITUDE
            } else {
                -(2.0 * TRIANGLE_AMPLITUDE / (period / 2.0)) * (n as f32 - period / 2.0)
                    + TRIANGLE_AMPLITUDE
            }
        })
        .cycle()
        .take(N)
        .collect::<heapless::Vec<f32, N>>();
    display("triangle signal", triangle.iter().cloned());

    //map it to real, leave im blank well fill in with cfft
    let mut dtfsecoef = triangle
        .iter()
        .cloned()
        .map(|f| Complex32 { re: f, im: 0.0 })
        .collect::<heapless::Vec<Complex32, N>>();

    // Coefficient calculation with CFFT function
    // arm_cfft_f32 uses a forward transform with enables bit reversal of output
    // well use microfft uses an in place Radix-2 FFT, for some reasons returns itself we dont need
    let _ = cfft(&mut dtfsecoef);
    println!("dtfsecoef: {:?}", &dtfsecoef);

    //dtfse to reclaim our original signal, note this is a bad approximation for our square wave
    let y_real = dtfse(dtfsecoef.iter().cloned(), 2).collect::<heapless::Vec<f32, N>>();
    display("y_real 2", y_real.iter().cloned());

    //a bit better
    let y_real = dtfse(dtfsecoef.iter().cloned(), 5).collect::<heapless::Vec<f32, N>>();
    display("y_real 5", y_real.iter().cloned());

    //good
    let y_real = dtfse(dtfsecoef.iter().cloned(), 8).collect::<heapless::Vec<f32, N>>();
    display("y_real 8", y_real.iter().cloned());

    //good
    let y_real = dtfse(dtfsecoef.iter().cloned(), 15).collect::<heapless::Vec<f32, N>>();
    display("y_real 15", y_real.iter().cloned());
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
        .map(|(idx, y)| (idx as f32, y))
        .collect::<Vec<(f32, f32)>>();
    Chart::new(120, 60, 0.0, N as f32)
        .lineplot(&Shape::Points(&display))
        .display();
}
