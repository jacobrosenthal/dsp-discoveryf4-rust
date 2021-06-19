//! This project is used for creating a digital sawtooth signal.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over to no_std without alloc.
//!
//! `cargo run --example 2_9`

use lab2::{display, Shape};

const N: usize = 100;
const SAW_AMPLITUDE: f32 = 0.75;
const SAW_PERIOD: usize = 20;

fn main() {
    // Collecting to turn the Cycle into a clean iterator for our naive display fn
    let sawtooth: heapless::Vec<f32, N> = (0..SAW_PERIOD)
        .map(|n| (2.0 * SAW_AMPLITUDE / (SAW_PERIOD as f32 - 1.0)) * n as f32 - SAW_AMPLITUDE)
        .cycle()
        .take(N)
        .collect();

    display("sawtooth signal", Shape::Line, sawtooth.iter().cloned());
}
