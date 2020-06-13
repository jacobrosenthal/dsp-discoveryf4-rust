//! This project is used for creating five different basic digital signals: unit
//! pulse, unit step, unit ramp, exponential and sinusoidal.
//!
//! `cargo run --example 2_1_basic_signals`

const N: usize = 10;
const A: f32 = 0.8;
const W0: f32 = core::f32::consts::PI / 5.0;

use textplots::{Chart, Plot, Shape};

fn main() {
    let mut unit_pulse = [0.0; N];
    unit_pulse.iter_mut().enumerate().for_each(|(idx, val)| {
        if idx == 0 {
            *val = 1.0;
        } else {
            *val = 0.0;
        }
    });
    display("unit_pulse", &unit_pulse[..]);

    let mut unit_step = [0.0; N];
    unit_step.iter_mut().for_each(|val| {
        *val = 1.0;
    });
    display("unit_step", &unit_step[..]);

    let mut unit_ramp = [0.0; N];
    unit_ramp
        .iter_mut()
        .enumerate()
        .for_each(|(idx, val)| *val = idx as f32);
    display("unit_ramp", &unit_ramp[..]);

    let mut exponential = [0.0; N];
    exponential
        .iter_mut()
        .enumerate()
        .for_each(|(idx, val)| *val = A.powf(idx as f32));
    display("exponential", &exponential[..]);

    let mut sinusoidal = [0.0; N];
    sinusoidal
        .iter_mut()
        .enumerate()
        .for_each(|(idx, val)| *val = (W0 * idx as f32).sin());
    display("sinusoidal", &sinusoidal[..]);
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
