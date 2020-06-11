//! This project is used for explaining IIR filtering operation using constant
//! coefficient difference equation.
//!
//! `cargo run --example 2_23_direct_iir_filtering`

use textplots::{Chart, Plot, Shape};

use core::f32::consts::{FRAC_PI_4, PI};

const N: usize = 512;

static B: &'static [f32] = &[0.002044, 0.004088, 0.002044];
static A: &'static [f32] = &[1f32, -1.819168, 0.827343];

fn main() {
    let mut x = [0f32; N];
    x.iter_mut()
        .enumerate()
        .for_each(|(n, val)| *val = (PI * n as f32 / 128.0).sin() + (FRAC_PI_4 * n as f32).sin());

    let mut y = [0f32; N];
    //random access of &mut y were iterating over.. so no iterators unless ... todo
    for y_idx in 0..N {
        y[y_idx] = B
            .iter()
            .enumerate()
            .map(|(coeff_idx, coeff)| {
                if coeff_idx < (y_idx + 1) {
                    coeff * x[y_idx - coeff_idx]
                } else {
                    0.0
                }
            })
            .sum::<f32>()
            + A.iter()
                .enumerate()
                .map(|(coeff_idx, coeff)| {
                    if coeff_idx < (y_idx + 1) {
                        -(coeff * y[y_idx - coeff_idx])
                    } else {
                        0.0
                    }
                })
                .sum::<f32>();
    }
    println!("y: {:?}", &y[..]);
    display(&y[..]);
}

// Note: Not ideal to Use lines over continuous, but only way to work on
// structures. Points does work, but small N doesnt lead to graphs that
// look like much. the seperate data structure to be combined later. If
// you have high enough resolution points can be good but n=10 isnt it
// Note: For input near origin, like unit pulse and step, points aren't
// discernable.
// Note: The as conversion could fail
// Note: Large N could blow stack I believe
fn display(input: &[f32]) {
    let display = input
        .iter()
        .enumerate()
        .map(|(idx, y)| (idx as f32, *y))
        .collect::<Vec<(f32, f32)>>();
    Chart::new(120, 60, 0f32, N as f32)
        .lineplot(Shape::Lines(&display[..]))
        .display();
}
