//! This project is used for creating two different digital periodic signals, a
//! square and triangle singal. The use of cycle to extend the basic signal
//! should emphasize the periodicity.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over no no_std without alloc.
//!
//! `cargo run --example 2_8`

use textplots::{Chart, Plot, Shape};

use heapless::consts::U100;
use itertools::Itertools;
use typenum::Unsigned;

const SQUARE_AMPLITUDE: f32 = 2.4;
const SQUARE_PERIOD: usize = 50;

const TRIANGLE_AMPLITUDE: f32 = 1.5;
const TRIANGLE_PERIOD: usize = 40;

fn main() {
    // Collecting to turn the Cycle into a clean iterator for our naive display fn
    let square = (0..SQUARE_PERIOD)
        .map(|idx| {
            if idx < (SQUARE_PERIOD / 2) {
                SQUARE_AMPLITUDE
            } else {
                -SQUARE_AMPLITUDE
            }
        })
        .cycle()
        .take(U100::to_usize())
        .collect::<heapless::Vec<f32, U100>>();
    display::<U100, _>("square signal", square.iter().cloned());

    // Collecting to turn the Cycle into a clean iterator for our naive display fn
    let triangle = (0..TRIANGLE_PERIOD)
        .map(|idx| {
            let period = TRIANGLE_PERIOD as f32;

            if idx < (TRIANGLE_PERIOD / 2) {
                (2.0 * TRIANGLE_AMPLITUDE / (period / 2.0)) * idx as f32 - TRIANGLE_AMPLITUDE
            } else {
                -(2.0 * TRIANGLE_AMPLITUDE / (period / 2.0)) * (idx as f32 - period / 2.0)
                    + TRIANGLE_AMPLITUDE
            }
        })
        .cycle()
        .take(U100::to_usize())
        .collect::<heapless::Vec<f32, U100>>();
    display::<U100, _>("triangle signal", triangle.iter().cloned());
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
        .lineplot(Shape::Steps(&display[..]))
        .display();
}
