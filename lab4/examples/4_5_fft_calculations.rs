//! This project is used for explaining the FFT operation using ARM CMSIS-DSP
//! library functions. Here we have a digital input signal, sum of two
//! sinusoidal signals with different frequencies. The complex form of this
//! signal is represented with s_complex array in main.c file. Frequency
//! components of this signal are found with arm_cfft_f32 function. Output of
//! this function is saved in the input array. The magnitude of the output
//! signal is calculated with the arm_cmplx_mag_f32 function. The result is
//! saved in the Mag array.
//!
//! Requires `cargo install probe-run`
//! `cargo run --release --example 4_5_fft_calculations`

#![no_std]
#![no_main]

use panic_break as _;
use stm32f4xx_hal as hal;

use cmsis_dsp_sys::{arm_cfft_f32, arm_cmplx_mag_f32};
use cty::uint32_t;
use hal::{dwt::ClockDuration, dwt::DwtExt, prelude::*, stm32};
use itertools::Itertools;
use micromath::F32Ext;
use rtt_target::{rprintln, rtt_init_print};
use typenum::{Sum, Unsigned};

use cmsis_dsp_sys::arm_cfft_sR_f32_len256 as arm_cfft_sR_f32;
type N = heapless::consts::U256;
type NCOMPLEX = Sum<N, N>;
///todo derive this from N
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

    //we wont use complex this time since, but just interleave the zeros for the imaginary part
    let mut s = s1
        .zip(s2)
        .map(|(ess1, ess2)| ess1 + ess2)
        .interleave_shortest(core::iter::repeat(0.0))
        .collect::<heapless::Vec<f32, NCOMPLEX>>();

    let mut mag = [0f32; N_CONST];

    let time: ClockDuration = dwt.measure(|| unsafe {
        //CFFT calculation
        arm_cfft_f32(&arm_cfft_sR_f32, s.as_mut_ptr(), 0, 1);

        // Magnitude calculation
        arm_cmplx_mag_f32(s.as_ptr(), mag.as_mut_ptr(), N::to_usize() as uint32_t);
    });
    rprintln!("ticks: {:?}", time.as_ticks());

    // signal to probe-run to exit
    loop {
        cortex_m::asm::bkpt()
    }
}

//C needs access to a sqrt fn, lets use micromath
#[no_mangle]
pub extern "C" fn sqrtf(x: f32) -> f32 {
    x.sqrt()
}
