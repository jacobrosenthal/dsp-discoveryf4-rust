//! This project is used for creating two different digital signals. One of
//! these signals is a periodic cosine wave and other one is aperiodic cosine
//! wave.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over to no_std without alloc.
//!
//! `cargo run --example 2_7_periodic_signals`

use lab2::{display, Shape};

const N: usize = 100;
const W1: f32 = core::f32::consts::PI / 10.0;
const W2: f32 = 3.0 / 10.0;

fn main() {
    let sinusoidal1 = (0..N).map(|n| (W1 * (n as f32)).cos());
    display("sinusoidal1", Shape::Line, sinusoidal1);

    let sinusoidal2 = (0..N).map(|n| (W2 * (n as f32)).cos());
    display("sinusoidal2", Shape::Line, sinusoidal2);
}
