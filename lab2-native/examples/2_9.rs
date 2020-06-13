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
    display("sawtooth period", &sawtooth[..]);

    // Generating the sawtooth signal
    for idx in 0..N {
        sawtooth[idx] = sawtooth[idx % SAW_PERIOD];
    }
    display("sawtooth signal", &sawtooth[..]);
}

// Points isn't a great representation as you can lose the line in the graph,
// however while Lines occasionally looks good it also can be terrible.
// Continuous requires to be in a fn pointer closure which cant capture any
// external data so not useful without lots of code duplication. Note: The as
// conversion could fail and passing large N slices could blow stack I believe
// because were passing as a slice
fn display(name: &str, input: &[f32]) {
    println!("{:?}: {:?}", name, &input[..]);
    let display = input
        .iter()
        .enumerate()
        .map(|(idx, y)| (idx as f32, *y))
        .collect::<Vec<(f32, f32)>>();
    Chart::new(120, 60, 0.0, N as f32)
        .lineplot(Shape::Points(&display[..]))
        .display();
}
