use micromath::F32Ext;
use typenum::Unsigned;
const A: f32 = 0.8;
const W0: f32 = core::f32::consts::PI / 5f32;

#[allow(unused)]
pub fn unit_pulse<N: Unsigned>() -> impl Iterator<Item = f32> {
    (0..(N::to_usize())).map(|val| if val == 0 { 1.0 } else { 0.0 })
}

#[allow(unused)]
pub fn unit_step<N: Unsigned>() -> impl Iterator<Item = f32> {
    (0..(N::to_usize())).map(|_| 1.0)
}

#[allow(unused)]
pub fn unit_ramp<N: Unsigned>() -> impl Iterator<Item = f32> {
    (0..(N::to_usize())).map(|val| val as f32)
}

#[allow(unused)]
pub fn sinusoidal<N: Unsigned>() -> impl Iterator<Item = f32> {
    (0..(N::to_usize())).map(|val| (W0 * val as f32).sin())
}

#[allow(unused)]
pub fn exponential<N: Unsigned>() -> impl Iterator<Item = f32> {
    (0..(N::to_usize())).map(|val| A.powf(val as f32))
}
