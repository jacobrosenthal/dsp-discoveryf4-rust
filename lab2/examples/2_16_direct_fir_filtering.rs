//! This project is used for measuring memory and execution time of FIR
//! filtering operation using convolution sum operation.
//!
//! Requires `cargo install probe-run`
//! `cargo run --release --example 2_16_direct_fir_filtering`
//!
//! Requires `cargo install cargo-binutils`
//! Requires `rustup component add llvm-tools-preview`
//! `cargo size --release --example 2_16_direct_fir_filtering`

#![no_std]
#![no_main]

use panic_break as _;
use stm32f4xx_hal as hal;

use core::f32::consts::{FRAC_PI_4, PI};
use hal::{dwt::ClockDuration, dwt::DwtExt, prelude::*, stm32};
use micromath::F32Ext;
use rtt_target::{rprintln, rtt_init_print};
use typenum::Unsigned;

type N = heapless::consts::U512;

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!(BlockIfFull);

    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // Set up the system clock.
    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz()) //discovery board has 8 MHz crystal for HSE
        .sysclk(168.mhz())
        .freeze();

    // Create a delay abstraction based on DWT cycle counter
    let dwt = cp.DWT.constrain(cp.DCB, clocks);

    let x =
        (0..N::to_usize()).map(|n| (PI * n as f32 / 128.0).sin() + (FRAC_PI_4 * n as f32).sin());

    let time: ClockDuration = dwt.measure(|| {
        //dificult to smuggle result out of the closure so dont bother.
        for _blah in convolution_sum(x.clone()).collect::<heapless::Vec<f32, N>>() {
            //hopefully this isnt optimized out since were not doing anything
        }
    });

    rprintln!("dft ticks: {:?}", time.as_ticks());

    // signal to probe-run to exit
    loop {
        cortex_m::asm::bkpt()
    }
}

pub fn convolution_sum<I>(x: I) -> impl Iterator<Item = f32> + Clone
where
    I: Iterator<Item = f32>
        + core::iter::ExactSizeIterator
        + core::iter::DoubleEndedIterator
        + Clone,
{
    (0..x.len()).map(move |y_n| {
        x.clone()
            .take(y_n + 1)
            .rev()
            .zip(H.iter())
            .map(|(exx, h)| h * exx)
            .sum()
    })
}

// low pass filter coefficients
static H: &[f32] = &[
    0.002044, 0.007806, 0.014554, 0.020018, 0.024374, 0.027780, 0.030370, 0.032264, 0.033568,
    0.034372, 0.034757, 0.034791, 0.034534, 0.034040, 0.033353, 0.032511, 0.031549, 0.030496,
    0.029375, 0.028207, 0.027010, 0.025800, 0.024587, 0.023383, 0.022195, 0.021031, 0.019896,
    0.018795, 0.017730, 0.016703, 0.015718, 0.014774, 0.013872, 0.013013, 0.012196, 0.011420,
    0.010684, 0.009989, 0.009331, 0.008711, 0.008127, 0.007577, 0.007061, 0.006575, 0.006120,
    0.005693, 0.005294, 0.004920, 0.004570, 0.004244, 0.003939, 0.003655, 0.003389, 0.003142,
    0.002912, 0.002698, 0.002499, 0.002313, 0.002141, 0.001981, 0.001833, 0.001695, 0.001567,
    0.001448,
];

// high pass filter coefficients for 2_18
// static H: &[f32] = &[
//     0.705514, -0.451674, -0.234801, -0.110490, -0.041705, -0.005635, 0.011617, 0.018401, 0.019652,
//     0.018216, 0.015686, 0.012909, 0.010303, 0.008042, 0.006173, 0.004677, 0.003506, 0.002605,
//     0.001922, 0.001409, 0.001028, 0.000746, 0.000540, 0.000389, 0.000279, 0.000200, 0.000143,
//     0.000102, 0.000072, 0.000051, 0.000036, 0.000026, 0.000018, 0.000013, 0.000009, 0.000006,
//     0.000004, 0.000003, 0.000002, 0.000002, 0.000001, 0.000001, 0.000001, 0.000000, 0.000000,
//     0.000000, 0.000000, 0.000000,
// ];
