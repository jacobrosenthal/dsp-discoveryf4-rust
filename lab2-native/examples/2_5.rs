//! This project is used for creating five different digital signals by applying
//! different operations on basic digital signals. You should observe that
//! digital signals can be modified using different arithmetic operations with
//! this application. Moreover, new digital signals can be obtained by combining
//! different digital signals.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over to no_std without alloc.
//!
//! `cargo run --example 2_5`

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

    // r[n]
    let unit_ramp = (0..N).map(|n| n as f32);

    // x1[n] =.6r[n+4]
    // I dont agree?... Book seems to think r[n+4] would be a window?
    let x1 = core::iter::repeat(0.0)
        .take(4)
        .chain(unit_ramp)
        .map(|dr| dr * 0.6);
    display("x1", Shape::Line, x1);

    // x2[n] = u[n-3]-u[n-8]
    let d3u = Delay::new(unit_step.clone(), 3);
    let d8u = Delay::new(unit_step.clone(), 8);
    let x2 = d3u.clone().zip(d8u.clone()).map(|(d3u, d8u)| d3u - d8u);
    display("x2", Shape::Line, x2.clone());

    // x3[n] = u[n]-u[n-3]+u[n-8]
    let x3 = unit_step
        .zip(d3u)
        .zip(d8u)
        .map(|((u, d3u), d8u)| u - d3u + d8u);
    display("x3", Shape::Line, x3);

    // x4[n] = x2[n]s[n]+d[n]
    let x4 = x2
        .zip(sinusoidal.clone().zip(unit_pulse))
        .map(|(x2, (s, d))| x2 * s + d);
    display("x4", Shape::Line, x4);

    // x5[n] = -2.4e[n]s[n]
    let x5 = exponential.zip(sinusoidal).map(|(e, s)| -2.4 * e * s);
    display("x5", Shape::Line, x5);
}

#[derive(Clone, Debug)]
struct Delay<I>
where
    I: Iterator<Item = f32>,
{
    delay: u32,
    n: u32,
    iter: I,
}

impl<I> Delay<I>
where
    I: Iterator<Item = f32>,
{
    fn new(iter: I, delay: u32) -> Self {
        Self { delay, n: 0, iter }
    }
}

impl<I> Iterator for Delay<I>
where
    I: Iterator<Item = f32>,
{
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.n < self.delay {
            self.n += 1;
            Some(0.0)
        } else {
            self.iter.next()
        }
    }
}
