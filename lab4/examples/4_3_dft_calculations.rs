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

use cmsis_dsp_sys::{arm_cmplx_mag_f32, arm_cos_f32, arm_sin_f32};
use core::f32::consts::PI;
use cty::{c_float, uint32_t};
use hal::{dwt::ClockDuration, dwt::DwtExt, prelude::*, stm32};
use micromath::F32Ext;
use typenum::Unsigned;

type N = heapless::consts::U256;
//todo derive this from N
const N_CONST: usize = 256;

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
    let s1 = (0..N::to_usize()).map(|val| (W1 * val as f32).sin());
    let s2 = (0..N::to_usize()).map(|val| (W2 * val as f32).sin());
    let s = s1.zip(s2).map(|(ess1, ess2)| ess1 + ess2);

    // map it to real, leave im blank well fill in with dft
    let dtfsecoef = s
        .map(|f| Complex32 { re: f, im: 0.0 })
        .collect::<heapless::Vec<Complex32, N>>();

    let mut mag = [0f32; N_CONST];

    let time: ClockDuration = dwt.measure(|| unsafe {
        let dft = dft::<N, _>(dtfsecoef.iter().cloned()).collect::<heapless::Vec<Complex32, N>>();

        // Magnitude calculation
        // a union of two f32 are just two f32 side by side in memory? so this
        // cast seems to be safe here and lets us keep using Complexf32 which is
        // far more ergonomic
        arm_cmplx_mag_f32(
            dft.as_ptr() as *const c_float,
            mag.as_mut_ptr(),
            N::to_usize() as uint32_t,
        );
    });
    rprintln!("ticks: {:?}", time.as_ticks());

    // signal to probe-run to exit
    loop {
        cortex_m::asm::bkpt()
    }
}

#[derive(Clone)]
struct Complex32 {
    re: f32,
    im: f32,
}

fn dft<N: Unsigned, I: Iterator<Item = Complex32> + Clone>(
    input: I,
) -> impl Iterator<Item = Complex32> {
    let size = N::to_usize() as f32;
    (0..N::to_usize()).map(move |k| {
        let k = k as f32;
        let mut sum_re = 0.0;
        let mut sum_im = 0.0;
        for (n, complex) in input.clone().enumerate() {
            let n = n as f32;
            sum_re += unsafe {
                complex.re * arm_cos_f32(2.0 * PI * k * n / size)
                    + complex.im * arm_sin_f32(2.0 * PI * k * n / size)
            };
            sum_im += unsafe {
                -complex.im * arm_cos_f32(2.0 * PI * k * n / size)
                    + complex.re * arm_sin_f32(2.0 * PI * k * n / size)
            };
        }

        Complex32 {
            re: sum_re,
            im: -sum_im,
        }
    })
}

//C needs access to a sqrt fn, lets use micromath
#[no_mangle]
pub extern "C" fn sqrtf(x: f32) -> f32 {
    x.sqrt()
}
