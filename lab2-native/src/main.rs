const W0: f32 = core::f32::consts::PI / 5f32;
// use micromath::F32Ext;
use micromath::{exp_ln2_approximation, ln_1to2_series_approximation};

fn main() {
    let s6 = (core::f32::consts::PI / 5f32 * 6.0).sin();
    dbg!(s6);

    // let bad = exp_ln2_approximation(2.0 * ln_1to2_series_approximation(sinusoidal), 4);
    // dbg!(bad);

    let good = s6.powf(2f32);
    dbg!(good);
}
