//! This project is used for explaining FIR filtering operation using CMSIS-DSP
//! library convolution function. Here we have a digital input signal, sum of
//! two sinusoidal signals with different frequencies. This signal is
//! represented with x array in main.c file. Originally, one of these sinusoidal
//! signals is filtered out using the coefficients given in
//! FIR_lpf_coefficients.h file. The output signal is represented with y array
//! and filter coefficients are stored in h array in main.c file. User can
//! replace FIR_lpf_coefficients.h file with FIR_hpf_coefficients.h file to
//! filter out other sinusoidal signal.
//!
//! Finally, replace (#include "FIR_lpf_coefficients.h") line with (#include
//! "FIR_hpf_coefficients.h") line in main.c file and repeat same steps for
//! obtaining second output array.  
//!
//! Requires `cargo install probe-run`
//! `cargo run --release --example 2_20_cmsis_convolution`

#![no_std]
#![no_main]

use panic_break as _;
use stm32f4xx_hal as hal;

use core::f32::consts::{FRAC_PI_4, PI};
use cty::{c_float, c_void, uint32_t};
use hal::{prelude::*, stm32};
use rtt_target::{rprintln, rtt_init_print};
use typenum::Unsigned;

type N = heapless::consts::U512;

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!(BlockIfFull);

    let dp = stm32::Peripherals::take().unwrap();
    let _cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // Set up the system clock.
    let rcc = dp.RCC.constrain();

    let _clocks = rcc
        .cfgr
        .use_hse(8.mhz()) //discovery board has 8 MHz crystal for HSE
        .sysclk(168.mhz())
        .freeze();

    let x = unsafe {
        (0..N::to_usize())
            .map(|n| arm_sin_f32(PI * n as f32 / 128.0) + arm_sin_f32(FRAC_PI_4 * n as f32))
            .collect::<heapless::Vec<f32, N>>()
    };

    // todo compute this. Length srcALen+srcBLen-1
    let mut y = [0f32; 512 + 64 - 1];

    unsafe {
        arm_conv_f32(
            x.as_ptr(),
            N::to_usize() as uint32_t,
            H.as_ptr(),
            H.len() as uint32_t,
            y.as_mut_ptr(),
        );
    }

    rprintln!("y: {:?}", &y[..]);

    // signal to probe-run to exit
    loop {
        cortex_m::asm::bkpt()
    }
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

// Converting CMSIS arm_math.h to expose prebuilt CMSIS
// libarm_cortexM4lf_math.lib static library linked via build.rs
// https://github.com/ARM-software/CMSIS_5 Todo auto convert these with bindgen
// and make a nice rusty library instead
extern "C" {
    /**
     * @brief  Fast approximation to the trigonometric sine function for floating-point data.
     * @param[in] x  input value in radians.
     * @return  sin(x).
     */
    fn arm_sin_f32(x: c_float) -> c_float;

    /**
     * @brief Convolution of floating-point sequences.
     * @param[in]  pSrcA    points to the first input sequence.
     * @param[in]  srcALen  length of the first input sequence.
     * @param[in]  pSrcB    points to the second input sequence.
     * @param[in]  srcBLen  length of the second input sequence.
     * @param[out] pDst     points to the location where the output result is written.  Length srcALen+srcBLen-1.
     */
    fn arm_conv_f32(
        pSrcA: *const c_float,
        srcALen: uint32_t,
        pSrcB: *const c_float,
        srcBLen: uint32_t,
        pDst: *mut c_float,
    ) -> c_void;

}
