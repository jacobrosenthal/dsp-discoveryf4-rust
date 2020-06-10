use micromath::F32Ext;

const A: f32 = 0.8;
const W0: f32 = core::f32::consts::PI / 5f32;

pub fn unit_pulse(range: core::ops::Range<usize>) -> impl Iterator<Item = f32> {
    range.map(|val| if val == 0 { 1.0 } else { 0.0 })
}

pub fn unit_step(range: core::ops::Range<usize>) -> impl Iterator<Item = f32> {
    range.map(|_| 1.0)
}

pub fn unit_ramp(range: core::ops::Range<usize>) -> impl Iterator<Item = f32> {
    range.map(|val| val as f32)
}

pub fn sinusoidal(range: core::ops::Range<usize>) -> impl Iterator<Item = f32> {
    range.map(|val| (W0 * val as f32).sin())
}

pub fn exponential(range: core::ops::Range<usize>) -> impl Iterator<Item = f32> {
    range.map(|val| A.powf(val as f32))
}
