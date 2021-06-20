//! This project is used for creating two different digital periodic signals, a
//! square and triangle singal. The use of cycle to extend the basic signal
//! should emphasize the periodicity.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over to no_std without alloc.
//!
//! `cargo run --example 2_8`

use lab2::{display, Shape};

const N: usize = 100;
const SQUARE_AMPLITUDE: f32 = 2.4;
const SQUARE_PERIOD: usize = 50;
const TRIANGLE_AMPLITUDE: f32 = 1.5;
const TRIANGLE_PERIOD: usize = 40;

fn main() {
    // Collecting to turn the Cycle into a clean iterator for our naive display fn
    let square: heapless::Vec<f32, N> = (0..SQUARE_PERIOD)
        .map(|n| {
            if n < (SQUARE_PERIOD / 2) {
                SQUARE_AMPLITUDE
            } else {
                -SQUARE_AMPLITUDE
            }
        })
        .cycle()
        .take(N)
        .collect();
    display("square signal", Shape::Line, square.iter().cloned());

    // Collecting to turn the Cycle into a clean iterator for our naive display fn
    let triangle: heapless::Vec<f32, N> = (0..TRIANGLE_PERIOD)
        .map(|n| {
            let period = TRIANGLE_PERIOD as f32;

            if n < (TRIANGLE_PERIOD / 2) {
                (2.0 * TRIANGLE_AMPLITUDE / (period / 2.0)) * n as f32 - TRIANGLE_AMPLITUDE
            } else {
                -(2.0 * TRIANGLE_AMPLITUDE / (period / 2.0)) * (n as f32 - period / 2.0)
                    + TRIANGLE_AMPLITUDE
            }
        })
        .cycle()
        .take(N)
        .collect();

    display("triangle signal", Shape::Line, triangle.iter().cloned());
}
