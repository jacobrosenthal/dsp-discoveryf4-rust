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
#![feature(array_from_fn)]

use panic_probe as _;
use stm32f4xx_hal as hal;

use cmsis_dsp_sys::{arm_cfft_f32, arm_cmplx_mag_f32};
use cty::{c_float, uint32_t};
use hal::{dwt::ClockDuration, dwt::DwtExt, prelude::*, stm32};
use micromath::F32Ext;
use rtt_target::{rprintln, rtt_init_print};

use cmsis_dsp_sys::arm_cfft_sR_f32_len256 as arm_cfft_sR_f32;
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

    // some sensor data source collected to an array so often
    // Complex sum of sinusoidal signals
    let s: [f32; N] = core::array::from_fn(|n| (W1 * n as f32).sin() + (W2 * n as f32).sin());

    let mut dtfse: [Complex32; N] = s.map(|v| Complex32 { re: v, im: 0.0 });

    let mut mag = [0f32; N];

    let time: ClockDuration = dwt.measure(|| unsafe {
        // CFFT calculation
        // Complex32 is repr(C) and f32 is float so should be able to cast to float array
        arm_cfft_f32(&arm_cfft_sR_f32, dtfse.as_mut_ptr() as *mut c_float, 0, 1);

        // Magnitude calculation
        arm_cmplx_mag_f32(s.as_ptr(), mag.as_mut_ptr(), N as uint32_t);
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

#[repr(C)]
#[derive(Clone, Debug)]
struct Complex32 {
    re: f32,
    im: f32,
}
