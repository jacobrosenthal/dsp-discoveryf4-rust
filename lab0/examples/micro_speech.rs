//! Tensorflow Demo.
//!
//! `cargo +nightly embed --release --example micro_speech`

#![no_std]
#![no_main]

use crate::hal::{dwt::DwtExt, prelude::*, stm32};
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal as hal;
use tfmicro::{MicroInterpreter, Model, MutableOpResolver};

#[entry]
fn main() -> ! {
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
    let mut delay = dwt.delay();

    let gpiod = dp.GPIOD.split();
    let mut led = gpiod.pd12.into_push_pull_output();

    let model = include_bytes!("../models/micro_speech.tflite");
    let no = include_bytes!("../models/no_micro_f9643d42_nohash_4.data");
    let yes = include_bytes!("../models/yes_micro_f2e59fea_nohash_1.data");

    // Map the model into a usable data structure. This doesn't involve
    // any copying or parsing, it's a very lightweight operation.
    let model = Model::from_buffer(&model[..]).unwrap();

    // Create an area of memory to use for input, output, and
    // intermediate arrays.
    const TENSOR_ARENA_SIZE: usize = 10 * 1024;
    let mut tensor_arena: [u8; TENSOR_ARENA_SIZE] = [0; TENSOR_ARENA_SIZE];

    // Pull in all needed operation implementations
    let micro_op_resolver = MutableOpResolver::empty()
        .depthwise_conv_2d()
        .fully_connected()
        .softmax();

    // Build an interpreter to run the model with
    let mut interpreter =
        MicroInterpreter::new(&model, micro_op_resolver, &mut tensor_arena[..]).unwrap();

    // Check properties of the input sensor
    assert_eq!([1, 49, 40, 1], interpreter.input_info(0).dims);

    // -------- 'yes' example --------
    interpreter.input(0, yes).unwrap();
    interpreter.invoke().unwrap();

    // Get output for 'yes'
    let output_tensor = interpreter.output(0);
    assert_eq!([1, 4], output_tensor.info().dims);

    let silence_score: u8 = output_tensor.as_data()[0];
    let unknown_score: u8 = output_tensor.as_data()[1];
    let yes_score: u8 = output_tensor.as_data()[2];
    let no_score: u8 = output_tensor.as_data()[3];

    assert!(yes_score > silence_score);
    assert!(yes_score > unknown_score);
    assert!(yes_score > no_score);

    // -------- 'no' example --------

    interpreter.input(0, no).unwrap();
    interpreter.invoke().unwrap();

    // Get output for 'no'
    let output_tensor = interpreter.output(0);
    assert_eq!([1, 4], output_tensor.info().dims);

    let silence_score: u8 = output_tensor.as_data()[0];
    let unknown_score: u8 = output_tensor.as_data()[1];
    let yes_score: u8 = output_tensor.as_data()[2];
    let no_score: u8 = output_tensor.as_data()[3];

    assert!(no_score > silence_score);
    assert!(no_score > unknown_score);
    assert!(no_score > yes_score);

    loop {
        led.set_high().ok();
        delay.delay_ms(333_u32);
        led.set_low().ok();
        delay.delay_ms(333_u32);
    }
}
