//! This project is used for creating a digital signal which is sum of two
//! sinusoidal signals with different frequencies.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over to no_std without alloc.
//!
//! `cargo run --example 2_6`

use core::f32::consts::{FRAC_PI_4, PI};
use textplots::{Chart, Plot, Shape};

const N: usize = 512;

fn main() {
    let w0 = (0..N).map(|n| (PI * n as f32 / 128.0).sin());

    let w1 = (0..N).map(|n| (FRAC_PI_4 * n as f32).sin());

    let y = w0.zip(w1).map(|(inny1, inny2)| inny1 + inny2);

    display("w1:", y);
}

// Points isn't a great representation as you can lose the line in the graph,
// however while Lines occasionally looks good it also can be terrible.
// Continuous requires to be in a fn pointer closure which cant capture any
// external data so not useful without lots of code duplication.
fn display<I>(name: &str, input: I)
where
    I: Iterator<Item = f32> + core::clone::Clone + std::fmt::Debug,
{
    println!("{:?}: ", name);
    let display: Vec<(f32, f32)> = input.enumerate().map(|(n, y)| (n as f32, y)).collect();
    Chart::new(120, 60, 0.0, N as f32)
        .lineplot(&Shape::Points(&display))
        .display();
}
