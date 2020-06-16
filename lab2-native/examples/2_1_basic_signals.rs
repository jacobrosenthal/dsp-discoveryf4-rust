//! This project is used for creating five different basic digital signals: unit
//! pulse, unit step, unit ramp, exponential and sinusoidal.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over no no_std without alloc.
//!
//! Demonstrates the use of using iter_mut() which will largely not be used
//! unless theres no other options. See 2_1_heapless_basic_signals
//!
//! `cargo run --example 2_1_basic_signals`

use textplots::{Chart, Plot, Shape};

use heapless::consts::U10;
use itertools::Itertools;
use typenum::Unsigned;

const A: f32 = 0.8;
const W0: f32 = core::f32::consts::PI / 5.0;

fn main() {
    let unit_pulse = (0..(U10::to_usize())).map(|idx| if idx == 0 { 1.0 } else { 0.0 });
    display::<U10, _>("unit_pulse", unit_pulse.clone());

    let unit_step = core::iter::repeat(1.0).take(U10::to_usize());
    display::<U10, _>("unit_step", unit_step.clone());

    let unit_ramp = (0..(U10::to_usize())).map(|idx| idx as f32);
    display::<U10, _>("unit_ramp", unit_ramp.clone());

    let exponential = (0..(U10::to_usize())).map(|idx| A.powf(idx as f32));
    display::<U10, _>("exponential", exponential.clone());

    let sinusoidal = (0..(U10::to_usize())).map(|idx| (W0 * idx as f32).sin());
    display::<U10, _>("sinusoidal", sinusoidal.clone());
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
