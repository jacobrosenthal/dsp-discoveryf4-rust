const N: usize = 10;
const A: f32 = 0.8;
const W0: f32 = core::f32::consts::PI / 5f32;

use textplots::{Chart, Plot, Shape};

fn main() {
    let mut unit_pulse = [0.0; N];
    unit_pulse.iter_mut().enumerate().for_each(|(idx, val)| {
        if idx == 0 {
            *val = 1.0;
        } else {
            *val = 0.0;
        }
    });
    println!("unit_pulse: {:?}", &unit_pulse[..]);
    display(&unit_pulse[..]);

    let mut unit_step = [0.0; N];
    unit_step.iter_mut().for_each(|val| {
        *val = 1.0;
    });
    println!("unit_step: {:?}", &unit_step[..]);
    display(&unit_step[..]);

    let mut unit_ramp = [0.0; N];
    unit_ramp
        .iter_mut()
        .enumerate()
        .for_each(|(idx, val)| *val = idx as f32);
    println!("unit_ramp: {:?}", &unit_ramp[..]);
    display(&unit_ramp[..]);

    let mut exponential = [0.0; N];
    exponential
        .iter_mut()
        .enumerate()
        .for_each(|(idx, val)| *val = A.powf(idx as f32));
    println!("exponential: {:?}", &exponential[..]);
    display(&exponential[..]);

    let mut sinusoidal = [0.0; N];
    sinusoidal
        .iter_mut()
        .enumerate()
        .for_each(|(idx, val)| *val = (W0 * idx as f32).sin());
    println!("sinusoidal: {:?}", &sinusoidal[..]);
    display(&sinusoidal[..]);
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
