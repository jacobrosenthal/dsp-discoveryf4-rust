//! This project is used for creating eight different digital signals by
//! applying different operations on basic digital signals.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over no no_std without alloc.
//!
//! `cargo run --example 2_4_operating_on_signals`

const N: usize = 10;
const A: f32 = 0.8;
const W0: f32 = core::f32::consts::PI / 5.0;

use textplots::{Chart, Plot, Shape};

fn main() {
    //unit pulse signal
    let mut unit_pulse = [0f32; N];
    unit_pulse[0] = 1.0;

    //unit step signal
    let unit_step = [1f32; N];

    //exponential signal
    let mut exponential = [0f32; N];
    exponential
        .iter_mut()
        .enumerate()
        .for_each(|(idx, val)| *val = A.powf(idx as f32));

    //sinusoidal signal
    let mut sinusoidal = [0f32; N];
    sinusoidal
        .iter_mut()
        .enumerate()
        .for_each(|(idx, val)| *val = (W0 * idx as f32).sin());

    //shifted unit pulse signal
    let mut x1 = [0f32; N];
    x1.iter_mut()
        .skip(4)
        .zip(&unit_pulse)
        .for_each(|(val, dee)| *val = *dee);
    display("x1", &x1[..]);

    //elevated sinusoidal signal
    let mut x2 = [0f32; N];
    x2.iter_mut()
        .zip(&sinusoidal)
        .for_each(|(val, ess)| *val = ess + 1.0);
    display("x2", &x2[..]);

    //negated unit step signal
    let mut x3 = [0f32; N];
    x3.iter_mut()
        .zip(&unit_step)
        .for_each(|(val, uu)| *val = -uu);
    display("x3", &x3[..]);

    //applying all operations on the sinusoidal signal
    let mut x4 = [0f32; N];
    x4.iter_mut()
        .skip(2)
        .zip(&sinusoidal)
        .for_each(|(val, ess)| *val = 3.0 * *ess - 2.0);
    display("x4", &x4[..]);

    //subtracting two unit step signals
    let mut x5 = [0f32; N];
    x5.iter_mut()
        .zip(&unit_step)
        .zip(unit_step.iter().skip(4))
        .enumerate()
        .for_each(|(idx, ((val, u1), udelay))| {
            if idx < 4 {
                *val = *u1 as f32;
            } else {
                *val = *u1 - udelay;
            }
        });
    display("x5", &x5[..]);

    // //multiplying the exponential signal with the unit step signal
    let mut x6 = [0f32; N];
    x6.iter_mut()
        .zip(&exponential)
        .zip(&unit_step)
        .for_each(|((val, e), u)| *val = e * *u as f32);
    display("x6", &x6[..]);

    // //multiplying the exponential signal with the sinusoidal signal
    let mut x7 = [0f32; N];
    x7.iter_mut()
        .zip(&exponential)
        .zip(&sinusoidal)
        .for_each(|((val, e), s)| *val = e * s);
    display("x7", &x7[..]);

    //multiplying the exponential signal with the window signal
    let mut x8 = [0f32; N];
    x8.iter_mut()
        .zip(&exponential)
        .zip(&x5)
        .for_each(|((val, e), x)| *val = e * *x as f32);
    display("x8", &x8[..]);
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
