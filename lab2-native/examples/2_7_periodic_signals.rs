//! This project is used for creating two different digital signals. One of
//! these signals is a periodic cosine wave and other one is aperiodic cosine
//! wave.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over to no_std without alloc.
//!
//! `cargo run --example 2_7_periodic_signals`

use textplots::{Chart, Plot, Shape};

const N: usize = 100;
const W1: f32 = core::f32::consts::PI / 10.0;
const W2: f32 = 3.0 / 10.0;

fn main() {
    let sinusoidal1 = (0..N).map(|n| (W1 * (n as f32)).cos());
    display("sinusoidal1", sinusoidal1);

    let sinusoidal2 = (0..N).map(|n| (W2 * (n as f32)).cos());
    display("sinusoidal2", sinusoidal2);
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
