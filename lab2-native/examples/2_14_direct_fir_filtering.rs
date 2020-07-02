//! This project is used for explaining FIR filtering operation using
//! convolution sum operation.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over no no_std without alloc.
//!
//! `cargo run --example 2_14_direct_fir_filtering`

use textplots::{Chart, Plot, Shape};

use core::f32::consts::{FRAC_PI_4, PI};
use heapless::consts::U512;
use itertools::Itertools;
use typenum::Unsigned;

// low pass filter coefficients
static H: &'static [f32] = &[
    0.002044, 0.007806, 0.014554, 0.020018, 0.024374, 0.027780, 0.030370, 0.032264, 0.033568,
    0.034372, 0.034757, 0.034791, 0.034534, 0.034040, 0.033353, 0.032511, 0.031549, 0.030496,
    0.029375, 0.028207, 0.027010, 0.025800, 0.024587, 0.023383, 0.022195, 0.021031, 0.019896,
    0.018795, 0.017730, 0.016703, 0.015718, 0.014774, 0.013872, 0.013013, 0.012196, 0.011420,
    0.010684, 0.009989, 0.009331, 0.008711, 0.008127, 0.007577, 0.007061, 0.006575, 0.006120,
    0.005693, 0.005294, 0.004920, 0.004570, 0.004244, 0.003939, 0.003655, 0.003389, 0.003142,
    0.002912, 0.002698, 0.002499, 0.002313, 0.002141, 0.001981, 0.001833, 0.001695, 0.001567,
    0.001448,
];

// high pass filter coefficients for 2_18
// static H: &'static [f32] = &[
//     0.705514, -0.451674, -0.234801, -0.110490, -0.041705, -0.005635, 0.011617, 0.018401, 0.019652,
//     0.018216, 0.015686, 0.012909, 0.010303, 0.008042, 0.006173, 0.004677, 0.003506, 0.002605,
//     0.001922, 0.001409, 0.001028, 0.000746, 0.000540, 0.000389, 0.000279, 0.000200, 0.000143,
//     0.000102, 0.000072, 0.000051, 0.000036, 0.000026, 0.000018, 0.000013, 0.000009, 0.000006,
//     0.000004, 0.000003, 0.000002, 0.000002, 0.000001, 0.000001, 0.000001, 0.000000, 0.000000,
//     0.000000, 0.000000, 0.000000,
// ];

fn main() {
    let x = (0..U512::to_usize())
        .map(|idx| (PI * idx as f32 / 128.0).sin() + (FRAC_PI_4 * idx as f32).sin());
    display::<U512, _>("y:", x.clone());

    // Collecting to have a clean iterator for our naive display fn
    let y = convolution_sum(x).collect::<heapless::Vec<f32, U512>>();
    display::<U512, _>("y:", y.iter().cloned());
}

pub fn convolution_sum<I>(x: I) -> impl Iterator<Item = f32> + Clone
where
    I: Iterator<Item = f32>
        + core::iter::ExactSizeIterator
        + core::iter::DoubleEndedIterator
        + Clone,
{
    (0..x.len()).map(move |y_idx| {
        x.clone()
            .take(y_idx + 1)
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
fn display<N, I>(name: &str, input: I)
where
    N: Unsigned,
    I: Iterator<Item = f32> + core::clone::Clone + std::fmt::Debug,
{
    println!("{:?}: {:?}", name, input.clone().format(", "));
    let display = input
        .enumerate()
        .map(|(idx, y)| (idx as f32, y))
        .collect::<Vec<(f32, f32)>>();
    Chart::new(120, 60, 0.0, N::to_usize() as f32)
        .lineplot(Shape::Points(&display[..]))
        .display();
}
