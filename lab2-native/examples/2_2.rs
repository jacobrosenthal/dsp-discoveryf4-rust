//! This project is used for creating two different digital sinusoidal signals
//! with certain frequencies.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over to no_std without alloc.
//!
//! `cargo run --example 2_2`

use core::f32::consts::{FRAC_PI_4, PI};
use lab2::{display, Shape};

const N: usize = 512;

fn main() {
    let w0: heapless::Vec<f32, N> = (0..N).map(|n| (PI * n as f32 / 128.0).sin()).collect();
    display("w0:", Shape::Line, w0.iter().cloned());

    let w1: heapless::Vec<f32, N> = (0..N).map(|n| (FRAC_PI_4 * n as f32).sin()).collect();
    display("w1:", Shape::Line, w1.iter().cloned());
}
