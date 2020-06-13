//! This project is used for creating eight different frame-based digital
//! systems.
//!
//! `cargo run --example 2_11_frame_based_systems`

use textplots::{Chart, Plot, Shape};

const N: usize = 10;
const W0: f32 = core::f32::consts::PI / 5.0;

fn digital_system1(b: f32, input: &[f32], output: &mut [f32]) {
    output
        .iter_mut()
        .zip(input)
        .for_each(|(out_ref, inny)| *out_ref = b * *inny)
}

fn digital_system2(input1: &[f32], input2: &[f32], output: &mut [f32]) {
    output
        .iter_mut()
        .zip(input1.iter().zip(input2))
        .for_each(|(out_ref, (inny1, inny2))| *out_ref = inny1 + inny2)
}

fn digital_system3(input: &[f32], output: &mut [f32]) {
    output
        .iter_mut()
        .zip(input)
        .for_each(|(out_ref, inny)| *out_ref = inny * inny)
}

fn digital_system4(b: &[f32], input: &[f32], output: &mut [f32]) {
    //random backwards access.. so no iterators unless ... todo
    for idx in 0..output.len() {
        if idx == 0 {
            output[idx] = b[0] * input[idx];
        } else {
            output[idx] = b[0] * input[idx] + b[1] * input[idx - 1];
        }
    }
}

fn digital_system5(b: &[f32], a: f32, input: &[f32], output: &mut [f32]) {
    //random backwards access.. so no iterators unless ... todo
    for idx in 0..output.len() {
        if idx == 0 {
            output[idx] = b[0] * input[idx];
        } else {
            output[idx] = b[0] * input[idx] + b[1] * input[idx - 1] + a * output[idx - 1];
        }
    }
}

fn digital_system6(b: &[f32], input: &[f32], output: &mut [f32]) {
    output
        .iter_mut()
        .zip(input.windows(2))
        .for_each(|(out_ref, inny)| *out_ref = b[0] * inny[1] + b[1] * inny[0])
}

fn digital_system7(b: f32, a: f32, input: &[f32], output: &mut [f32]) {
    //random backwards access.. so no iterators unless ... todo
    for idx in 0..output.len() {
        if idx == 0 {
            output[idx] = b * input[idx];
        } else {
            output[idx] = b * input[idx] + a * output[idx - 1];
        }
    }
}

fn digital_system8(input: &[f32], output: &mut [f32]) {
    output
        .iter_mut()
        .zip(input)
        .enumerate()
        .for_each(|(idx, (out_ref, inny))| *out_ref = idx as f32 * inny)
}

fn main() {
    //unit step signal
    let unit_step = [1f32; N];

    // unit pulse
    let mut unit_pulse = [0f32; N];
    unit_pulse.iter_mut().enumerate().for_each(|(idx, val)| {
        if idx == 0 {
            *val = 1.0;
        } else {
            *val = 0.0;
        }
    });

    //sinusoidal signal
    let mut sinusoidal = [0f32; N];
    sinusoidal
        .iter_mut()
        .enumerate()
        .for_each(|(idx, val)| *val = (W0 * idx as f32).sin());

    //y[n] = b x[n]
    let mut y1 = [0f32; N];
    digital_system1(2.2, &unit_step, &mut y1);
    display("digital_system1", &y1[..]);

    //y[n] = x1[n] + x2[n]
    let mut y2 = [0f32; N];
    digital_system2(&unit_step, &sinusoidal, &mut y2);
    display("digital_system2", &y2[..]);

    //y[n] = x^2[n]
    let mut y3 = [0f32; N];
    digital_system3(&sinusoidal, &mut y3);
    display("digital_system3", &y3[..]);

    //y[n] = b0 x[n] + b1 x[n-1]
    let mut y4 = [0f32; N];
    digital_system4(&[2.2, -1.1], &sinusoidal, &mut y4);
    display("digital_system4", &y4[..]);

    //y[n] = b0 x[n] + b1 x[n-1] + a1 y[n-1]
    let mut y5 = [0f32; N];
    digital_system5(&[2.2, -1.1], 0.7, &sinusoidal, &mut y5);
    display("digital_system5", &y5[..]);

    //y[n] = b0 x[n+1] + b1 x[n]
    //digital_system6 in c version has oob array access, should be if (n+1 < size) so y6[9] undefined
    let mut y6 = [0f32; N];
    digital_system6(&[2.2, -1.1], &unit_step, &mut y6);
    display("digital_system6", &y6[..]);

    //y[n] = b0 x[n] + a1 y[n-1]
    let mut y7 = [0f32; N];
    digital_system7(1.0, 2.0, &unit_pulse, &mut y7);
    display("digital_system7", &y7[..]);

    //y[n] = n x[n]
    let mut y8 = [0f32; N];
    digital_system8(&sinusoidal, &mut y8);
    display("digital_system8", &y8[..]);
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
    Chart::new(120, 60, 0f32, N as f32)
        .lineplot(Shape::Points(&display[..]))
        .display();
}
