const N: usize = 100;
const SAW_AMPLITUDE: f32 = 0.75;
const SAW_PERIOD: usize = 20;

use textplots::{Chart, Plot, Shape};

fn main() {
    // One period of the sawtooth signal
    let mut sawtooth = [0f32; N];
    sawtooth.iter_mut().enumerate().for_each(|(idx, val)| {
        *val = (2.0 * SAW_AMPLITUDE / (SAW_PERIOD as f32 - 1.0)) * idx as f32 - SAW_AMPLITUDE;
    });
    println!("sawtooth period: {:?}", &sawtooth[..]);
    display(&sawtooth[..]);

    // Generating the sawtooth signal
    for idx in 0..N {
        sawtooth[idx] = sawtooth[idx % SAW_PERIOD];
    }
    println!("sawtooth signal: {:?}", &sawtooth[..]);
    display(&sawtooth[..]);
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
    Chart::new(120, 60, 0.0, N as f32)
        .lineplot(Shape::Lines(&display[..]))
        .display();
}
