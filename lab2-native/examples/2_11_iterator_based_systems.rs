//! This project is used for creating eight different frame-based digital
//! systems.
//!
//! This rust implementation isnt particularly frame based, nor is it sample
//! based. Both are very unrusty as they rely on array access with either bounds
//! checking performance costs or no bounds checking when in release mode and
//! then common UB with invald array access. Instead this is largely an iterator
//! based approach. These can be easily developed inline,  except where
//! impossible to implement because of needing random access, theyre implemented
//! as an impl Iterator on a custom struct.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over no no_std without alloc.
//!
//! `cargo run --example 2_11_iterator_based_systems`

use textplots::{Chart, Plot, Shape};

use heapless::consts::U10;
use itertools::Itertools;
use typenum::Unsigned;

const W0: f32 = core::f32::consts::PI / 5.0;

fn main() {
    // d[n]
    let unit_pulse = (0..(U10::to_usize())).map(|val| if val == 0 { 1.0 } else { 0.0 });

    // u[n]
    let unit_step = (0..(U10::to_usize())).map(|_| 1.0);

    // s[n]
    let sinusoidal = (0..(U10::to_usize())).map(|val| (W0 * val as f32).sin());

    // multiplier
    // y[n] = b*x[n]
    let y1 = unit_step.clone().map(|u| 2.2 * u);
    display::<U10, _>("digital_system1", y1.clone());

    // adder accumulator
    // y[n] = x1[n] + x2[n]
    let y2 = sinusoidal
        .clone()
        .zip(unit_step.clone())
        .map(|(inny1, inny2)| inny1 + inny2);
    display::<U10, _>("digital_system2", y2.clone());

    // squaring device
    // y[n] = x^2[n]
    let y3 = sinusoidal.clone().map(|inny| inny * inny);
    display::<U10, _>("digital_system3", y3.clone());

    // multiplier and accumulator
    // y[n] = b0*x[n] + b1*x[n-1]
    let delay_sin = Delay::new(sinusoidal.clone(), 1);
    let y4 = sinusoidal
        .clone()
        .map(|s| 2.2 * s)
        .zip(delay_sin.map(|ds| ds * -1.1))
        .map(|(a, b)| a + b);
    display::<U10, _>("digital_system4", y4.clone());

    // multiplier and accumulator with feedback
    // y[n] = b0*x[n] + b1*x[n-1] + a*y[n-1]
    let y5 = DigitalSystem5::new(sinusoidal.clone());
    display::<U10, _>("digital_system5", y5.clone());

    // multiplier and accumulator with future input
    // y[n] = b0*x[n+1] + b1*x[n]
    // digital_system6 in c version has oob array access, should be if (n+1 < size) so y6[9] undefined
    let y6 = unit_step
        .clone()
        .tuple_windows()
        .map(|(u0, u1)| 2.2 * u1 + -1.1 * u0);
    display::<U10, _>("digital_system6", y6.clone());

    // multiplier and accumulator with unbounded output
    // y[n] = b0*x[n] + b1*y[n-1]
    let y7 = DigitalSystem7::new(unit_pulse.clone());
    display::<U10, _>("digital_system7", y7.clone());

    // multiplier with a time based coefficient
    // y[n]=n*x[n]
    let y8 = sinusoidal
        .clone()
        .enumerate()
        .map(|(idx, inny)| idx as f32 * inny);
    display::<U10, _>("digital_system8", y8.clone());
}

#[derive(Clone, Debug)]
struct Delay<I>
where
    I: Iterator<Item = f32>,
{
    delay: u32,
    idx: u32,
    iter: I,
}

impl<I> Delay<I>
where
    I: Iterator<Item = f32>,
{
    fn new(iter: I, delay: u32) -> Self {
        Self {
            delay,
            idx: 0,
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
        if self.idx < self.delay {
            self.idx += 1;
            Some(0.0)
        } else {
            self.iter.next()
        }
    }
}

/// y[n] = b0*x[n] + b1*x[n-1] + a*y[n-1]
#[derive(Clone, Debug)]
struct DigitalSystem5<I>
where
    I: Iterator<Item = f32>,
{
    last_in: Option<f32>,
    last_out: Option<f32>,
    iter: I,
}

impl<I> DigitalSystem5<I>
where
    I: Iterator<Item = f32>,
{
    fn new(iter: I) -> Self {
        Self {
            last_in: None,
            last_out: None,
            iter: iter,
        }
    }
}

impl<I> Iterator for DigitalSystem5<I>
where
    I: Iterator<Item = f32>,
{
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if let Some(val) = self.iter.next() {
            let abc = if let (Some(last_in), Some(last_out)) = (self.last_in, self.last_out) {
                2.2 * val + -1.1 * last_in + 0.7 * last_out
            } else {
                2.2 * val
            };

            self.last_in = Some(val);
            self.last_out = Some(abc);

            Some(abc)
        } else {
            None
        }
    }
}

/// y[n] = b0*x[n] + b1*y[n-1]
#[derive(Clone, Debug)]
struct DigitalSystem7<I>
where
    I: Iterator<Item = f32>,
{
    last_out: Option<f32>,
    iter: I,
}

impl<I> DigitalSystem7<I>
where
    I: Iterator<Item = f32>,
{
    fn new(iter: I) -> Self {
        Self {
            last_out: None,
            iter: iter,
        }
    }
}
impl<I> Iterator for DigitalSystem7<I>
where
    I: Iterator<Item = f32>,
{
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if let Some(val) = self.iter.next() {
            self.last_out = if let Some(last_out) = self.last_out {
                Some(1.0 * val + 2.0 * last_out)
            } else {
                Some(1.0 * val)
            };

            self.last_out
        } else {
            None
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
        .map(|(idx, y)| (idx as f32, y))
        .collect::<Vec<(f32, f32)>>();
    Chart::new(120, 60, 0.0, N::to_usize() as f32)
        .lineplot(Shape::Points(&display[..]))
        .display();
}
