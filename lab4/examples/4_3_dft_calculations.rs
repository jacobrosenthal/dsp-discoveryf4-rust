//! This project is used for explaining the DFT operation using standard math
//! functions. Here, we have a digital input signal as the sum of two sinusoids
//! with different frequencies. The complex form of this signal is represented
//! with s_complex array, the frequency component of this signal is found by the
//! DFT function. Real and imaginary parts of the obtained DFT are represented
//! with XR and XI arrays. The magnitude of DFT is kept in the Mag array.
//!
//! Requires `cargo install probe-run`
//! `cargo run --release --example 4_3_dft_calculations`
//!
//! I think its supposed to be faster with their functions, and their sin/cos is
//! decently faster.. but something about bringing our sqrt across for them and
//! our ability to optimize pure rust is making us faster actually!

#![no_std]
#![no_main]

use panic_break as _;
use stm32f4xx_hal as hal;

use cmsis_dsp_sys::arm_cmplx_mag_f32;
use core::f32::consts::PI;
use cty::{c_float, uint32_t};
use hal::{dwt::ClockDuration, dwt::DwtExt, prelude::*, stm32};
use micromath::F32Ext;
use rtt_target::{rprintln, rtt_init_print};

const N: usize = 256;

const W1: f32 = core::f32::consts::PI / 128.0;
const W2: f32 = core::f32::consts::PI / 4.0;
// const W2: f32 = core::f32::consts::PI / 5.0;

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!(BlockIfFull, 128);

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

    // Complex sum of sinusoidal signals
    let s1 = (0..N).map(|val| (W1 * val as f32).sin());
    let s2 = (0..N).map(|val| (W2 * val as f32).sin());
    let s = s1.zip(s2).map(|(ess1, ess2)| ess1 + ess2);

    // map it to real, leave im blank well fill in with dft
    let dtfsecoef = s
        .map(|f| Complex32 { re: f, im: 0.0 })
        .collect::<heapless::Vec<Complex32, N>>();

    let mut mag = [0f32; N];

    let time: ClockDuration = dwt.measure(|| unsafe {
        let dft = dft(dtfsecoef.into_iter()).collect::<heapless::Vec<Complex32, N>>();

        // Magnitude calculation
        // a union of two f32 are just two f32 side by side in memory? so this
        // cast seems to be safe here and lets us keep using Complexf32 which is
        // far more ergonomic
        arm_cmplx_mag_f32(
            dft.as_ptr() as *const c_float,
            mag.as_mut_ptr(),
            N as uint32_t,
        );
    });
    rprintln!("ticks: {:?}", time.as_ticks());

    // signal to probe-run to exit
    loop {
        cortex_m::asm::bkpt()
    }
}

fn dft<I: Iterator<Item = Complex32> + Clone>(input: I) -> impl Iterator<Item = Complex32> {
    let size = N as f32;
    (0..N).map(move |k| {
        input
            .clone()
            .enumerate()
            .fold((0f32, 0f32), |(mut sum_re, mut sum_im), (n, complex)| {
                let n = n as f32;
                sum_re += complex.re * (2.0 * PI * k as f32 * n / size).cos()
                    + complex.im * (2.0 * PI * k as f32 * n / size).sin();
                sum_im += -complex.im * (2.0 * PI * k as f32 * n / size).cos()
                    + complex.re * (2.0 * PI * k as f32 * n / size).sin();

                (sum_re, sum_im)
            })
            .into()
    })
}

#[derive(Clone)]
struct Complex32 {
    re: f32,
    im: f32,
}

impl From<(f32, f32)> for Complex32 {
    fn from(incoming: (f32, f32)) -> Self {
        Complex32 {
            re: incoming.0,
            im: incoming.1,
        }
    }
}

//C needs access to a sqrt fn, lets use micromath
#[no_mangle]
pub extern "C" fn sqrtf(x: f32) -> f32 {
    x.sqrt()
}
