//! This project is used for creating five different digital signals by applying
//! different operations on basic digital signals. You should observe that
//! digital signals can be modified using different arithmetic operations with
//! this application. Moreover, new digital signals can be obtained by combining
//! different digital signals.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over no no_std without alloc.
//!
//! `cargo run --example 2_5`

use textplots::{Chart, Plot, Shape};

use heapless::consts::U10;
use itertools::Itertools;
use typenum::Unsigned;

const A: f32 = 0.8;
const W0: f32 = core::f32::consts::PI / 5f32;

fn main() {
    // d[n]
    let unit_pulse = (0..(U10::to_usize())).map(|val| if val == 0 { 1.0 } else { 0.0 });

    // u[n]
    let unit_step = core::iter::repeat(1.0).take(U10::to_usize());

    // e[n]
    let exponential = (0..(U10::to_usize())).map(|val| A.powf(val as f32));

    // s[n]
    let sinusoidal = (0..(U10::to_usize())).map(|val| (W0 * val as f32).sin());

    // r[n]
    let unit_ramp = (0..(U10::to_usize())).map(|n| n as f32);

    // x1[n] =.6r[n+4]
    // I dont agree?... Book seems to think r[n+4] would be a window?
    let x1 = core::iter::repeat(0.0)
        .take(4)
        .chain(unit_ramp.clone())
        .map(|dr| dr * 0.6);
    display::<U10, _>("x1", x1.clone());

    // x2[n] = u[n-3]-u[n-8]
    let d3u = Delay::new(unit_step.clone(), 3);
    let d8u = Delay::new(unit_step.clone(), 8);
    let x2 = d3u.clone().zip(d8u.clone()).map(|(d3u, d8u)| d3u - d8u);
    display::<U10, _>("x2", x2.clone());

    // x3[n] = u[n]-u[n-3]+u[n-8]
    let x3 = unit_step
        .clone()
        .zip(d3u.clone())
        .zip(d8u.clone())
        .map(|((u, d3u), d8u)| u - d3u + d8u);
    display::<U10, _>("x3", x3.clone());

    // x4[n] = x2[n]s[n]+d[n]
    let x4 = x2
        .clone()
        .zip(sinusoidal.clone().zip(unit_pulse))
        .map(|(x2, (s, d))| x2 * s + d);
    display::<U10, _>("x4", x4.clone());

    // x5[n] = -2.4e[n]s[n]
    let x5 = exponential
        .clone()
        .zip(sinusoidal.clone())
        .map(|(e, s)| -2.4 * e * s);
    display::<U10, _>("x5", x5.clone());
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
        Self {
            delay,
            n: 0,
            iter: iter,
        }
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

// Points isn't a great representation as you can lose the line in the graph,
// however while Lines occasionally looks good it also can be terrible.
// Continuous requires to be in a fn pointer closure which cant capture any
// external data so not useful without lots of code duplication.
fn display<N, I>(name: &str, input: I)
where
    N: Unsigned,
    I: Iterator<Item = f32> + core::clone::Clone + std::fmt::Debug,
{
    println!("{:?}: {:?}", name, input.clone().format(", "));
    let display = input
        .enumerate()
        .map(|(n, y)| (n as f32, y))
        .collect::<Vec<(f32, f32)>>();
    Chart::new(120, 60, 0.0, N::to_usize() as f32)
        .lineplot(Shape::Points(&display[..]))
        .display();
}
