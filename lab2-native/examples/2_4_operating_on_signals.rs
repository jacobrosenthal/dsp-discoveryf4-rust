//! This project is used for creating eight different digital signals by
//! applying different operations on basic digital signals.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over to no_std without alloc.
//!
//! `cargo run --example 2_4_operating_on_signals`

use lab2::{display, Shape};

const N: usize = 10;
const A: f32 = 0.8;
const W0: f32 = core::f32::consts::PI / 5f32;

fn main() {
    // d[n]
    let unit_pulse = (0..N).map(|val| if val == 0 { 1.0 } else { 0.0 });

    // u[n]
    let unit_step = core::iter::repeat(1.0).take(N);

    // e[n]
    let exponential = (0..N).map(|val| A.powf(val as f32));

    // s[n]
    let sinusoidal = (0..N).map(|val| (W0 * val as f32).sin());

    // shifted unit pulse signal u[n+3]
    let x1 = core::iter::repeat(0.0).take(3).chain(unit_pulse).take(N);
    display("x1", Shape::Points, x1);

    // elevated sinusoidal s[n]+1.0
    let x2 = sinusoidal.clone().map(|ess| ess + 1.0);
    display("x2", Shape::Line, x2);

    // negated unit step -u[n]
    let x3 = unit_step.clone().map(|us| -us);
    display("x3", Shape::Line, x3);

    // applying all operations on the sinusoidal signal
    // I disagree with the book on this, x4[0] and x4[1] would be -2 shifted
    let x4 = core::iter::repeat(0.0)
        .take(2)
        .chain(sinusoidal.clone())
        .take(N)
        .map(|ess| 3.0 * ess - 2.0);
    display("x4", Shape::Line, x4);

    // subtracting two unit step signals
    let x5 = core::iter::repeat(0.0)
        .take(4)
        .chain(unit_step.clone())
        .take(N)
        .zip(unit_step.clone())
        .map(|(us_delay, us)| us - us_delay);
    display("x5", Shape::Points, x5.clone());

    // multiplying the exponential signal with the unit step signal
    let x6 = exponential.clone().zip(unit_step).map(|(ex, us)| ex * us);
    display("x6", Shape::Line, x6);

    // multiplying the exponential signal with the sinusoidal signal
    let x7 = exponential.clone().zip(sinusoidal).map(|(ex, ss)| ex * ss);
    display("x7", Shape::Line, x7);

    // multiplying the exponential signal with the window signal
    let x8 = exponential.zip(x5).map(|(ex, x5)| ex * x5);
    display("x8", Shape::Points, x8);
}
