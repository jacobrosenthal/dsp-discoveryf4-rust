//! This project is used for creating a digital signal which is sum of two
//! sinusoidal signals with different frequencies.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over to no_std without alloc.
//!
//! `cargo run --example 2_6`

use core::f32::consts::{FRAC_PI_4, PI};
use lab2::{display, Shape};

const N: usize = 512;

fn main() {
    let w0 = (0..N).map(|n| (PI * n as f32 / 128.0).sin());

    let w1 = (0..N).map(|n| (FRAC_PI_4 * n as f32).sin());

    let y = w0.zip(w1).map(|(inny1, inny2)| inny1 + inny2);

    display("w1:", Shape::Line, y);
}
