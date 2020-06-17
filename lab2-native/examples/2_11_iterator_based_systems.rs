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
//! `cargo run --example 2_11_frame_based_systems`

use textplots::{Chart, Plot, Shape};

use heapless::consts::U10;
use itertools::Itertools;
use typenum::Unsigned;

const W0: f32 = core::f32::consts::PI / 5.0;

fn main() {
    //unit pulse signal
    let unit_pulse = (0..(U10::to_usize())).map(|val| if val == 0 { 1.0 } else { 0.0 });

    //unit step signal
    let unit_step = (0..(U10::to_usize())).map(|_| 1.0);

    //sinusoidal signal
    let sinusoidal = (0..(U10::to_usize())).map(|val| (W0 * val as f32).sin());

    //y[n] = b x[n]
    let y1 = unit_step.clone().map(|u| 2.2 * u);
    display::<U10, _>("digital_system1", y1.clone());

    //y[n] = x1[n] + x2[n]
    let y2 = sinusoidal
        .clone()
        .zip(unit_step.clone())
        .map(|(inny1, inny2)| inny1 + inny2);
    display::<U10, _>("digital_system2", y2.clone());

    //y[n] = x^2[n]
    let y3 = sinusoidal.clone().map(|inny| inny * inny);
    display::<U10, _>("digital_system3", y3.clone());

    //y[n] = b0 x[n] + b1 x[n-1]
    let y4 = DigitalSystem4::new(sinusoidal.clone());
    display::<U10, _>("digital_system4", y4.clone());

    //y[n] = b0 x[n] + b1 x[n-1] + a1 y[n-1]
    let y5 = DigitalSystem5::new(sinusoidal.clone());
    display::<U10, _>("digital_system5", y5.clone());

    //y[n] = b0 x[n+1] + b1 x[n]
    // digital_system6 in c version has oob array access, should be if (n+1 < size) so y6[9] undefined
    let y6 = unit_step
        .clone()
        .tuple_windows()
        .map(|(u0, u1)| 2.2 * u1 + -1.1 * u0);
    display::<U10, _>("digital_system6", y6.clone());

    //y[n] = b0 x[n] + a1 y[n-1]
    let y7 = DigitalSystem7::new(unit_pulse.clone());
    display::<U10, _>("digital_system7", y7.clone());

    //y[n] = n x[n]
    let y8 = sinusoidal
        .clone()
        .enumerate()
        .map(|(idx, inny)| idx as f32 * inny);
    display::<U10, _>("digital_system8", y8.clone());
}

#[derive(Clone, Debug)]
struct DigitalSystem4<I>
where
    I: Iterator<Item = f32>,
{
    last: Option<f32>,
    iter: I,
}

impl<I> DigitalSystem4<I>
where
    I: Iterator<Item = f32>,
{
    fn new(iter: I) -> Self {
        Self {
            last: None,
            iter: iter,
        }
    }
}

impl<I> Iterator for DigitalSystem4<I>
where
    I: Iterator<Item = f32>,
{
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if let Some(val) = self.iter.next() {
            let abc = if let Some(last) = self.last {
                2.2 * val + -1.1 * last
            } else {
                2.2 * val
            };

            self.last = Some(val);
            Some(abc)
        } else {
            None
        }
    }
}

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
                2.2 * val + -1.1 * last_in + -1.1 * last_out
            } else {
                2.2 * val
            };

            self.last_in = Some(val);
            Some(abc)
        } else {
            None
        }
    }
}

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
