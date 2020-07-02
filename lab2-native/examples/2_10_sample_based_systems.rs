//! This project is used for creating eight different sample-based digital
//! systems.
//!
//! This sample based example is less compile time checked but allows random
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
//! `cargo run --example 2_10_sample_based_systems`

use textplots::{Chart, Plot, Shape};

const N: usize = 10;
const W0: f32 = core::f32::consts::PI / 5.0;

fn digital_system1(b: f32, input: f32) -> f32 {
    b * input
}

fn digital_system2(input1: f32, input2: f32) -> f32 {
    input1 + input2
}

fn digital_system3(input: f32) -> f32 {
    input * input
}

fn digital_system4(b: &[f32], input0: f32, input1: f32) -> f32 {
    b[0] * input0 + b[1] * input1
}

fn digital_system5(b: &[f32], a: f32, input0: f32, input1: f32, output: f32) -> f32 {
    b[0] * input0 + b[1] * input1 + a * output
}

fn digital_system6(b: &[f32], input0: f32, input1: f32) -> f32 {
    b[0] * input0 + b[1] * input1
}

fn digital_system7(b: f32, a: f32, input: f32, output: f32) -> f32 {
    b * input + a * output
}

fn digital_system8(n: f32, input: f32) -> f32 {
    n * input
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

    //sinusoidal signal
    let mut sinusoidal = [0f32; N];
    sinusoidal
        .iter_mut()
        .enumerate()
        .for_each(|(n, val)| *val = (W0 * n as f32).sin());

    // multiplier
    // y[n] = b*x[n]
    let mut y1 = [0f32; N];
    for n in 0..N {
        y1[n] = digital_system1(2.2, unit_step[n]);
    }
    display("digital_system1", &y1[..]);

    // adder accumulator
    // y[n] = x1[n] + x2[n]
    let mut y2 = [0f32; N];
    for n in 0..N {
        y2[n] = digital_system2(unit_step[n], sinusoidal[n]);
    }
    display("digital_system2", &y2[..]);

    // squaring device
    // y[n] = x^2[n]
    let mut y3 = [0f32; N];
    for n in 0..N {
        y3[n] = digital_system3(sinusoidal[n]);
    }
    display("digital_system3", &y3[..]);

    // multiplier and accumulator
    // y[n] = b0*x[n] + b1*x[n-1]
    let mut y4 = [0f32; N];
    for n in 0..N {
        if n == 0 {
            y4[n] = digital_system4(&[2.2, -1.1], sinusoidal[n], 0.0);
        } else {
            y4[n] = digital_system4(&[2.2, -1.1], sinusoidal[n], sinusoidal[n - 1]);
        }
    }
    display("digital_system4", &y4[..]);

    // multiplier and accumulator with feedback
    // y[n] = b0*x[n] + b1*x[n-1] + a*y[n-1]
    let mut y5 = [0f32; N];
    for n in 0..N {
        if n == 0 {
            y5[n] = digital_system5(&[2.2, -1.1], 0.7, sinusoidal[n], 0.0, 0.0);
        } else {
            y5[n] = digital_system5(
                &[2.2, -1.1],
                0.7,
                sinusoidal[n],
                sinusoidal[n - 1],
                y5[n - 1],
            );
        }
    }
    display("digital_system5", &y5[..]);

    // multiplier and accumulator with future input
    // y[n] = b0*x[n+1] + b1*x[n]
    // digital_system6 in c version has oob array access, so y6[9] 0
    let mut y6 = [0f32; N];
    for n in 0..N {
        // digital_system6 in c version has oob array access, should be if (n+1 < size)
        if n + 1 < N {
            y6[n] = digital_system6(&[2.2, -1.1], unit_step[n + 1], unit_step[n]);
        }
    }
    display("digital_system6", &y6[..]);

    // multiplier and accumulator with unbounded output
    // y[n] = b0*x[n] + b1*y[n-1]
    let mut y7 = [0f32; N];
    for n in 0..N {
        if n == 0 {
            y7[n] = digital_system7(1.0, 2.0, unit_pulse[n], 0.0);
        } else {
            y7[n] = digital_system7(1.0, 2.0, unit_pulse[n], y7[n - 1]);
        }
    }
    display("digital_system7", &y7[..]);

    // multiplier with a time based coefficient
    // y[n]=n*x[n]
    let mut y8 = [0f32; N];
    for n in 0..N {
        y8[n] = digital_system8(n as f32, sinusoidal[n]);
    }
    display("digital_system8", &y8[..]);
}

// Points isn't a great representation as you can lose the line in the graph,
// however while Lines occasionally looks good it also can be terrible.
// Continuous requires to be in a fn pointer closure which cant capture any
// external data so not useful without lots of code duplication.
fn display(name: &str, input: &[f32]) {
    println!("{:?}: {:.4?}", name, &input[..]);
    let display = input
        .iter()
        .enumerate()
        .map(|(n, y)| (n as f32, *y))
        .collect::<Vec<(f32, f32)>>();
    Chart::new(120, 60, 0.0, input.len() as f32)
        .lineplot(Shape::Points(&display[..]))
        .display();
}
