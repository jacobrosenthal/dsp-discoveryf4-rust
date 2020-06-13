//! This project is used for creating eight different digital signals by
//! applying different operations on basic digital signals.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over no no_std without alloc.
//!
//! `cargo run --example 2_7_periodic_signals`

const N: usize = 100;
const W1: f32 = core::f32::consts::PI / 10.0;
const W2: f32 = 3.0 / 10.0;

use textplots::{Chart, Plot, Shape};

fn main() {
    let mut sinusoidal1 = [0f32; N];
    sinusoidal1
        .iter_mut()
        .enumerate()
        .for_each(|(idx, val)| *val = (W1 * (idx as f32)).cos());
    display("sinusoidal1", &sinusoidal1[..]);

    let mut sinusoidal2 = [0f32; N];
    sinusoidal2
        .iter_mut()
        .enumerate()
        .for_each(|(idx, val)| *val = (W2 * (idx as f32)).cos());
    display("sinusoidal2", &sinusoidal2[..]);
}

// Points isn't a great representation as you can lose the line in the graph,
// however while Lines occasionally looks good it also can be terrible.
// Continuous requires to be in a fn pointer closure which cant capture any
// external data so not useful without lots of code duplication. Note: The as
// conversion could fail and passing large N slices could blow stack I believe
// because were passing as a slice
fn display(name: &str, input: &[f32]) {
    println!("{:?}: {:?}", name, &input[..]);
    let display = input
        .iter()
        .enumerate()
        .map(|(idx, y)| (idx as f32, *y))
        .collect::<Vec<(f32, f32)>>();
    Chart::new(120, 60, 0.0, N as f32)
        .lineplot(Shape::Points(&display[..]))
        .display();
}
