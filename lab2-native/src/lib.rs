use itertools::Itertools;
use textplots::{Chart, Plot};

#[non_exhaustive]
pub enum Shape {
    Line,
    Points,
}

pub fn display<I>(name: &str, shape: Shape, input: I)
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

    let data = match shape {
        Shape::Line => textplots::Shape::Lines(&display),
        Shape::Points => textplots::Shape::Points(&display),
    };

    let n = display.len();
    let width = 256;

    // Continuous requires to be in a fn pointer closure which cant capture any
    // external data so not useful without lots of code duplication.
    // Lines occasionally looks good.. but mostly bad
    Chart::new(width as u32, 60, 0.0, n as f32)
        .lineplot(&data)
        .display();
}
