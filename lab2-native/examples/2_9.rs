//! This project is used for creating a digital sawtooth signal.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over no no_std without alloc.
//!
//! `cargo run --example 2_9`

use itertools::Itertools;
use textplots::{Chart, Plot, Shape};

const N: usize = 100;

const SAW_AMPLITUDE: f32 = 0.75;
const SAW_PERIOD: usize = 20;

fn main() {
    // Collecting to turn the Cycle into a clean iterator for our naive display fn
    let sawtooth = (0..SAW_PERIOD)
        .map(|n| (2.0 * SAW_AMPLITUDE / (SAW_PERIOD as f32 - 1.0)) * n as f32 - SAW_AMPLITUDE)
        .cycle()
        .take(N)
        .collect::<heapless::Vec<f32, N>>();

    display("sawtooth signal", sawtooth.iter().cloned());
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
        .map(|(n, y)| (n as f32, y))
        .collect::<Vec<(f32, f32)>>();
    Chart::new(120, 60, 0.0, N as f32)
        .lineplot(Shape::Lines(&display[..]))
        .display();
}
