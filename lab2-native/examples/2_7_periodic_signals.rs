//! This project is used for creating eight different digital signals by
//! applying different operations on basic digital signals.
//!
//! `cargo run --release --example 2_7_periodic_signals`

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
    println!("sinusoidal1: {:?}", &sinusoidal1[..]);
    display(&sinusoidal1[..]);

    let mut sinusoidal2 = [0f32; N];
    sinusoidal2
        .iter_mut()
        .enumerate()
        .for_each(|(idx, val)| *val = (W2 * (idx as f32)).cos());
    println!("sinusoidal2: {:?}", &sinusoidal2[..]);
    display(&sinusoidal2[..]);
}

// Note: Not ideal to Use lines over continuous, but only way to work on
// structures. Points does work, but small N doesnt lead to graphs that
// look like much. the seperate data structure to be combined later. If
// you have high enough resolution points can be good but n=10 isnt it
// Note: For input near origin, like unit pulse and step, points aren't
// discernable.
// Note: The as conversion could fail
// Note: Large N could blow stack I believe
fn display(input: &[f32]) {
    let display = input
        .iter()
        .enumerate()
        .map(|(idx, y)| (idx as f32, *y))
        .collect::<Vec<(f32, f32)>>();
    Chart::new(120, 60, 0.0, N as f32)
        .lineplot(Shape::Lines(&display[..]))
        .display();
}
