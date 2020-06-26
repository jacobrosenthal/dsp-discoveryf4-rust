//! This project is used for creating a digital sawtooth signal.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over no no_std without alloc.
//!
//! `cargo run --example 2_9`

use textplots::{Chart, Plot, Shape};

use heapless::consts::U100;
use itertools::Itertools;
use typenum::Unsigned;

const SAW_AMPLITUDE: f32 = 0.75;
const SAW_PERIOD: usize = 20;

fn main() {
    // Collecting to turn the Cycle into a clean iterator for our naive display fn
    let sawtooth = (0..SAW_PERIOD)
        .map(|idx| (2.0 * SAW_AMPLITUDE / (SAW_PERIOD as f32 - 1.0)) * idx as f32 - SAW_AMPLITUDE)
        .cycle()
        .take(U100::to_usize())
        .collect::<heapless::Vec<f32, U100>>();

    display::<U100, _>("sawtooth signal", sawtooth.iter().cloned());
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
