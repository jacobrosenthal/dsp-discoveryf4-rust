//! More examples on sample-based and frame-based implementation of digital
//! systems. I implemented these as iterator based, as usual.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over to no_std without alloc.
//!
//! `cargo run --example 2_12`

use itertools::Itertools;
use lab2::display;

const N: usize = 10;
const A: f32 = 0.8;

fn main() {
    // e[n]
    let exponential = (0..N).map(|val| A.powf(val as f32));

    // r[n]
    let unit_ramp = (0..N).map(|n| n as f32);

    // y1[n]=x1[n]+x2[n], where x1[n]=r[n] and x2[n]=e[n]
    let y1 = unit_ramp.clone().zip(exponential).map(|(r, e)| r + e);
    display("y1", y1.clone());

    // y2[n]=x3[n], where x3[n]=r^2[n]
    let y2 = unit_ramp.clone().zip(unit_ramp).map(|(r, rr)| r * rr);
    display("y2", y2.clone());

    // y3[n]=2.2y1[n]-1.1y1[n-1]+.7y3[n-1]
    let y3 = DigitalSystem5::new(y1);
    display("y3", y3);

    // y4[n]=2.2y2[n+1]-1.1y2[n]
    let y4 = y2.tuple_windows().map(|(y2, y2_1)| 2.2 * y2_1 - 1.1 * y2);
    display("y4", y4);
}

// y3[n]=2.2y1[n]-1.1y1[n-1]+.7y3[n-1]
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
            iter,
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
            let out = if let (Some(last_in), Some(last_out)) = (self.last_in, self.last_out) {
                2.2 * val + -1.1 * last_in + 0.7 * last_out
            } else {
                2.2 * val
            };

            self.last_in = Some(val);
            self.last_out = Some(out);

            Some(out)
        } else {
            None
        }
    }
}
