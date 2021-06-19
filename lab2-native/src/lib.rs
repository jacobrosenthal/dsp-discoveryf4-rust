use itertools::Itertools;
use textplots::{Chart, Plot, Shape};

pub fn display<I>(name: &str, input: I)
where
    I: IntoIterator,
    <I as IntoIterator>::IntoIter: Clone,
    <I as std::iter::IntoIterator>::Item: Into<f32> + std::fmt::Debug,
{
    let i = input.into_iter();
    let display: Vec<(f32, f32)> = i
        .clone()
        .enumerate()
        .map(|(n, y)| (n as f32, y.into()))
        .collect();
    println!("{:?}: {:.4?}", name, i.format(", "));

    // Continuous requires to be in a fn pointer closure which cant capture any
    // external data so not useful without lots of code duplication.
    // Lines occasionally looks good.. but mostly bad
    Chart::new(120, 60, 0.0, display.len() as f32)
        .lineplot(&Shape::Points(&display))
        .display();
}
