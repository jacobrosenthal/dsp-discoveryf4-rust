//! This project is used for creating five different basic digital signals: unit
//! pulse, unit step, unit ramp, exponential and sinusoidal.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over to no_std without alloc.
//!
//! `cargo run --example 2_1_basic_signals`

use lab2::{display, Shape};

const N: usize = 10;
const A: f32 = 0.8;
const W0: f32 = core::f32::consts::PI / 5.0;

fn main() {
    // d[n]
    let unit_pulse = (0..N).map(|n| if n == 0 { 1.0 } else { 0.0 });
    display("unit_pulse", Shape::Line, unit_pulse);

    // u[n]
    let unit_step = core::iter::repeat(1.0).take(N);
    display("unit_step", Shape::Line, unit_step);

    // r[n]
    let unit_ramp = (0..N).map(|n| n as f32);
    display("unit_ramp", Shape::Line, unit_ramp);

    // e[n]
    let exponential = (0..N).map(|n| A.powf(n as f32));
    display("exponential", Shape::Line, exponential);

    // s[n]
    let sinusoidal = (0..N).map(|n| (W0 * n as f32).sin());
    display("sinusoidal", Shape::Line, sinusoidal);
}
