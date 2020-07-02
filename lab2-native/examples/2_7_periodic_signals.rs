//! This project is used for creating two different digital signals. One of
//! these signals is a periodic cosine wave and other one is aperiodic cosine
//! wave.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over no no_std without alloc.
//!
//! `cargo run --example 2_7_periodic_signals`

use textplots::{Chart, Plot, Shape};
use typenum::Unsigned;

type N = heapless::consts::U100;

const W1: f32 = core::f32::consts::PI / 10.0;
const W2: f32 = 3.0 / 10.0;

fn main() {
    let sinusoidal1 = (0..(N::to_usize())).map(|n| (W1 * (n as f32)).cos());
    display::<N, _>("sinusoidal1", sinusoidal1.clone());

    let sinusoidal2 = (0..(N::to_usize())).map(|n| (W2 * (n as f32)).cos());
    display::<N, _>("sinusoidal2", sinusoidal2.clone());
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
    println!("{}", name);
    let display = input
        .enumerate()
        .map(|(n, y)| (n as f32, y))
        .collect::<Vec<(f32, f32)>>();
    Chart::new(120, 60, 0.0, N::to_usize() as f32)
        .lineplot(Shape::Points(&display[..]))
        .display();
}
