//! This project is used for explaining IIR filtering operation using constant
//! coefficient difference equation.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over no no_std without alloc.
//!
//! `cargo run --example 2_23_direct_iir_filtering`

use textplots::{Chart, Plot, Shape};

use core::f32::consts::{FRAC_PI_4, PI};
use heapless::consts::U512;
use itertools::Itertools;
use typenum::Unsigned;

const N: usize = 512;

// high pass filter coefficients
static B: &'static [f32] = &[0.002044, 0.004088, 0.002044];
static A: &'static [f32] = &[1.0, -1.819168, 0.827343];

// low pass filter coefficients for 2_24
// static B: &'static [f32] = &[0.705514, -1.411028, 0.705514];
// static A: &'static [f32] = &[1.0, -1.359795, 0.462261];

fn main() {
    let x = (0..U512::to_usize())
        .map(|idx| (PI * idx as f32 / 128.0).sin() + (FRAC_PI_4 * idx as f32).sin())
        .collect::<heapless::Vec<f32, U512>>();
    display::<U512, _>("x:", x.iter().cloned());

    //random access of &mut y were iterating over.. so no iterators unless ... todo
    let mut y = [0f32; N];
    for y_idx in 0..N {
        y[y_idx] = B
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
                        -(coeff * y[y_idx - coeff_idx])
                    } else {
                        0.0
                    }
                })
                .sum::<f32>();
    }
    display::<U512, _>("y:", y.iter().cloned());
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
