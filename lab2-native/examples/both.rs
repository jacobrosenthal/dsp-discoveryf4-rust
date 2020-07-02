//! This project is used for explaining IIR filtering operation using constant
//! coefficient difference equation.
//!
//! Requires cargo embed
//! `cargo install cargo-embed`
//!
//! `cargo embed --example 2_23_direct_iir_filtering`

use textplots::{Chart, Plot, Shape};

use core::f32::consts::{FRAC_PI_4, PI};
use heapless::consts::U512;
// use itertools::Itertools;
use typenum::Unsigned;

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

const N: usize = 512;
static B: &'static [f32] = &[0.002044, 0.004088, 0.002044];
static A: &'static [f32] = &[1f32, -1.819168, 0.827343];

fn main() {
    let x = (0..U512::to_usize())
        .map(|idx| (PI * idx as f32 / 128.0).sin() + (FRAC_PI_4 * idx as f32).sin())
        .collect::<heapless::Vec<f32, U512>>();
    // display::<U512, _>("x:", x.iter().cloned());

    //random access of &mut y were iterating over.. so no iterators unless ... todo
    let mut y1 = [0f32; N];
    for y_idx in 0..N {
        y1[y_idx] = B
            .iter()
            .enumerate()
            .map(|(coeff_idx, coeff)| {
                if coeff_idx < (y_idx + 1) {
                    coeff * x[y_idx - coeff_idx]
                } else {
                    0.0
                }
            })
            .sum::<f32>()
            + A.iter()
                .enumerate()
                .map(|(coeff_idx, coeff)| {
                    if coeff_idx < (y_idx + 1) {
                        -(coeff * y1[y_idx - coeff_idx])
                    } else {
                        0.0
                    }
                })
                .sum::<f32>();
    }
    // display::<U512, _>("y:", y.iter().cloned());

    // Collecting to have a clean iterator for our naive display fn
    let y2 = convolution_sum(x.iter().cloned()).collect::<heapless::Vec<f32, U512>>();

    let yy = y1
        .iter()
        .zip(y2.clone())
        .map(|(inny1, inny2)| inny1 + inny2)
        .collect::<heapless::Vec<f32, U512>>();

    display::<U512, _>("yy:", yy.iter().cloned());
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
fn display<N, I>(_name: &str, input: I)
where
    N: Unsigned,
    I: Iterator<Item = f32> + core::clone::Clone + std::fmt::Debug,
{
    // println!("{:?}: {:?}", name, input.clone().format(", "));
    let display = input
        .enumerate()
        .map(|(idx, y)| (idx as f32, y))
        .collect::<Vec<(f32, f32)>>();
    Chart::new(120, 60, 0.0, N::to_usize() as f32)
        .lineplot(Shape::Points(&display[..]))
        .display();
}
