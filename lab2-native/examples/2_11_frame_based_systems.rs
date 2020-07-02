//! This project is used for creating eight different frame-based digital
//! systems.
//!
//! This frame based example is less compile time checked but allows random
//! access which is impossible in iterator approaches and easier than
//! implementing an iterator from scratch. Note the bounds checking is elided
//! for speed in release mode so make sure while developing you're running in
//! debug mode to catch and fix panics due to out of bounds access See the
//! prefered iterator based approaches which will be used more commonly from
//! here on out.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over no no_std without alloc.
//!
//! `cargo run --example 2_11_frame_based_systems`

use textplots::{Chart, Plot, Shape};

const N: usize = 10;
const W0: f32 = core::f32::consts::PI / 5.0;

fn digital_system1(b: f32, input: &[f32], output: &mut [f32]) {
    for n in 0..output.len() {
        output[n] = b * input[n];
    }
}

fn digital_system2(input1: &[f32], input2: &[f32], output: &mut [f32]) {
    for n in 0..output.len() {
        output[n] = input1[n] + input2[n];
    }
}

fn digital_system3(input: &[f32], output: &mut [f32]) {
    for n in 0..output.len() {
        output[n] = input[n] * input[n];
    }
}

fn digital_system4(b: &[f32], input: &[f32], output: &mut [f32]) {
    for n in 0..output.len() {
        if n == 0 {
            output[n] = b[0] * input[n];
        } else {
            output[n] = b[0] * input[n] + b[1] * input[n - 1];
        }
    }
}

fn digital_system5(b: &[f32], a: f32, input: &[f32], output: &mut [f32]) {
    for n in 0..output.len() {
        if n == 0 {
            output[n] = b[0] * input[n];
        } else {
            output[n] = b[0] * input[n] + b[1] * input[n - 1] + a * output[n - 1];
        }
    }
}

fn digital_system6(b: &[f32], input: &[f32], output: &mut [f32]) {
    for n in 0..output.len() {
        // digital_system6 in c version has oob array access, should be if (n+1 < size)
        if n + 1 < input.len() {
            output[n] = b[0] * input[n + 1] + b[1] * input[n];
        }
    }
}

fn digital_system7(b: f32, a: f32, input: &[f32], output: &mut [f32]) {
    for n in 0..output.len() {
        if n == 0 {
            output[n] = b * input[n];
        } else {
            output[n] = b * input[n] + a * output[n - 1];
        }
    }
}

fn digital_system8(input: &[f32], output: &mut [f32]) {
    for n in 0..output.len() {
        output[n] = n as f32 * input[n];
    }
}

fn main() {
    // d[n]
    let mut unit_pulse = [0f32; N];
    unit_pulse.iter_mut().enumerate().for_each(|(n, val)| {
        if n == 0 {
            *val = 1.0;
        } else {
            *val = 0.0;
        }
    });

    // u[n]
    let unit_step = [1f32; N];

    // s[n]
    let mut sinusoidal = [0f32; N];
    sinusoidal
        .iter_mut()
        .enumerate()
        .for_each(|(n, val)| *val = (W0 * n as f32).sin());

    // multiplier
    // y[n] = b*x[n]
    let mut y1 = [0f32; N];
    digital_system1(2.2, &unit_step, &mut y1);
    display("digital_system1", &y1[..]);

    // adder accumulator
    // y[n] = x1[n] + x2[n]
    let mut y2 = [0f32; N];
    digital_system2(&unit_step, &sinusoidal, &mut y2);
    display("digital_system2", &y2[..]);

    // squaring device
    // y[n] = x^2[n]
    let mut y3 = [0f32; N];
    digital_system3(&sinusoidal, &mut y3);
    display("digital_system3", &y3[..]);

    // multiplier and accumulator
    // y[n] = b0*x[n] + b1*x[n-1]
    let mut y4 = [0f32; N];
    digital_system4(&[2.2, -1.1], &sinusoidal, &mut y4);
    display("digital_system4", &y4[..]);

    // multiplier and accumulator with feedback
    // y[n] = b0*x[n] + b1*x[n-1] + a*y[n-1]
    let mut y5 = [0f32; N];
    digital_system5(&[2.2, -1.1], 0.7, &sinusoidal, &mut y5);
    display("digital_system5", &y5[..]);

    // multiplier and accumulator with future input
    // y[n] = b0*x[n+1] + b1*x[n]
    // digital_system6 in c version has oob array access, so y6[9] 0
    let mut y6 = [0f32; N];
    digital_system6(&[2.2, -1.1], &unit_step, &mut y6);
    display("digital_system6", &y6[..]);

    // multiplier and accumulator with unbounded output
    // y[n] = b0*x[n] + b1*y[n-1]
    let mut y7 = [0f32; N];
    digital_system7(1.0, 2.0, &unit_pulse, &mut y7);
    display("digital_system7", &y7[..]);

    // multiplier with a time based coefficient
    // y[n]=n*x[n]
    let mut y8 = [0f32; N];
    digital_system8(&sinusoidal, &mut y8);
    display("digital_system8", &y8[..]);
}

// Points isn't a great representation as you can lose the line in the graph,
// however while Lines occasionally looks good it also can be terrible.
// Continuous requires to be in a fn pointer closure which cant capture any
// external data so not useful without lots of code duplication.
fn display(name: &str, input: &[f32]) {
    println!("{:?}: {:?}", name, &input[..]);
    let display = input
        .iter()
        .enumerate()
        .map(|(n, y)| (n as f32, *y))
        .collect::<Vec<(f32, f32)>>();
    Chart::new(120, 60, 0.0, input.len() as f32)
        .lineplot(Shape::Points(&display[..]))
        .display();
}
