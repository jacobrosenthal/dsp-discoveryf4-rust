//! This project is used for creating two different digital sinusoidal signals
//! with certain frequencies.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over to no_std without alloc.
//!
//! `cargo run --example 2_2`
#![feature(array_from_fn)]

use core::f32::consts::{FRAC_PI_4, PI};
use lab2::{display, Shape};

const N: usize = 512;

fn main() {
    let w0: [f32; N] = core::array::from_fn(|n| (PI * n as f32 / 128.0).sin());
    display("w0:", Shape::Line, w0.iter().cloned());

    let w1: [f32; N] = core::array::from_fn(|n| (FRAC_PI_4 * n as f32).sin());
    display("w1:", Shape::Line, w1.iter().cloned());
}
