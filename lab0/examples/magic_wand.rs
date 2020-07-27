//! Port of Tensorflow Gesture Demo
//! https://learn.adafruit.com/tensorflow-lite-for-edgebadge-kit-quickstart/gesture-demo
//!
//! Board components facing up, holding the usb like a wand, but level with the
//! ground, when the led turns green perform one of three gestures:
//!
//! Wing: This gesture is a W starting at your top left, going down, up, down up
//! to your top right When that gesture is detected you'lll see the front
//! led turn orange.
//!
//! Ring: This gesture is a O starting at top center, then moving clockwise in a
//! circle to the right, then down, then left and back to when you started in
//! the top center When that gesture is detected you'll see the front led
//! turn red.
//!
//! Slope: This gesture is an L starting at your top right, moving diagonally to
//! your bottom left, then straight across to bottom right. When that gesture is
//! detected you'll see the front ked turn light blue.
//!
//! Setup:
//! * figure out how to install arm-none-eabi-gcc for your os
//! * `rustup update` to get a recent nightly near august
//!
//! `cargo +nightly embed --release --example magic_wand`

#![no_std]
#![no_main]

use panic_rtt_target as _;
use stm32f4xx_hal as hal;
// use panic_break as _;

use hal::spi;
use hal::{dwt::DwtExt, prelude::*, stm32};
use lis3dsh::{accelerometer::RawAccelerometer, Lis3dsh};
use rtt_target::{rprintln, rtt_init, set_print_channel};
use tfmicro::{MicroInterpreter, Model, MutableOpResolver};

#[cfg(not(feature = "tf_train"))]
const N: usize = 128;

#[cfg(feature = "tf_train")]
const N: usize = 64;

#[cortex_m_rt::entry]
fn main() -> ! {
    let channels = rtt_init! {
        up: {
            0: {
                size: 1024
                mode: BlockIfFull
                name: "Text"
            }
            1: {
                size: 128
                mode: BlockIfFull
                name: "Graph"
            }
        }
    };
    set_print_channel(channels.up.0);
    let mut graph = channels.up.1;

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

    let gpioa = dp.GPIOA.split();
    let gpioe = dp.GPIOE.split();
    let gpiod = dp.GPIOD.split();

    let mut green = gpiod.pd12.into_push_pull_output();
    let mut orange = gpiod.pd13.into_push_pull_output();
    let mut red = gpiod.pd14.into_push_pull_output();
    let mut blue = gpiod.pd15.into_push_pull_output();

    let sck = gpioa.pa5.into_alternate_af5().internal_pull_up(false);
    let miso = gpioa.pa6.into_alternate_af5().internal_pull_up(false);
    let mosi = gpioa.pa7.into_alternate_af5().internal_pull_up(false);

    let spi_mode = spi::Mode {
        polarity: spi::Polarity::IdleHigh,
        phase: spi::Phase::CaptureOnSecondTransition,
    };

    let spi = spi::Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        spi_mode,
        10.mhz().into(),
        clocks,
    );

    let chip_select = gpioe.pe3.into_push_pull_output();
    let mut lis3dsh = Lis3dsh::new_spi(spi, chip_select);
    lis3dsh.init(&mut delay).unwrap();
    lis3dsh.set_range(lis3dsh::Range::G4).unwrap();
    lis3dsh.set_datarate(lis3dsh::DataRate::Hz_25).unwrap();

    let model = include_bytes!("../models/magic_wand.tflite");

    let test = include_bytes!("../models/ring_micro_f9643d42_nohash_4.data")
        .chunks_exact(4)
        .map(|c| f32::from_be_bytes([c[0], c[1], c[2], c[3]]))
        .collect::<heapless::Vec<_, heapless::consts::U384>>();

    // Map the model into a usable data structure. This doesn't involve
    // any copying or parsing, it's a very lightweight operation.
    let model = Model::from_buffer(&model[..]).unwrap();

    // Create an area of memory to use for input, output, and
    // intermediate arrays.
    const TENSOR_ARENA_SIZE: usize = 60 * 1024;
    let mut tensor_arena: [u8; TENSOR_ARENA_SIZE] = [0; TENSOR_ARENA_SIZE];

    // Pull in all needed operation implementations
    let micro_op_resolver = MutableOpResolver::empty()
        .depthwise_conv_2d()
        .max_pool_2d()
        .conv_2d()
        .fully_connected()
        .softmax();

    // Build an interpreter to run the model with
    let mut interpreter =
        MicroInterpreter::new(&model, micro_op_resolver, &mut tensor_arena[..]).unwrap();

    // Check properties of the input sensor
    assert_eq!([1, 128, 3, 1], interpreter.input_info(0).dims);

    //  (x,y,z)
    let mut data = [0.0; N * 3];

    loop {
        rprintln!("Magic starts!");
        green.set_high().ok();

        (0..N).for_each(|n| {
            while !lis3dsh.is_data_ready().unwrap() {}
            let dat = lis3dsh.accel_raw().unwrap();

            // test data is normalized to 1mg per digit
            // 4g .12 mg/digit

            let x = dat[0] as f32 * 0.12;
            let y = dat[1] as f32 * 0.12;
            let z = dat[2] as f32 * 0.12;

            // the first element is stable and negative in the test data, for us thats then y

            data[n * 3] = -y; //z
            data[n * 3 + 1] = z; //y
            data[n * 3 + 2] = x; //x
        });

        rprintln!("{:?}", data);
        data.iter().for_each(|f| {
            graph.write(&f.to_le_bytes());
        });

        interpreter.input(0, &data).ok();

        interpreter.invoke().ok();

        let output_tensor = interpreter.output(0);
        assert_eq!([1, 4], output_tensor.info().dims);

        let res = output_tensor.as_data::<f32>();
        rprintln!("{:.4?}", res);

        // 0 WingScore
        // 1 RingScore
        // 2 SlopeScore
        // 3 NegativeScore
        if res[0] > 0.5 {
            orange.set_high().ok();
        } else if res[1] > 0.5 {
            red.set_high().ok();
        } else if res[2] > 0.5 {
            blue.set_high().ok();
        };
        green.set_low().ok();

        delay.delay_ms(1000_u32);
        delay.delay_ms(1000_u32);
        delay.delay_ms(1000_u32);

        orange.set_low().ok();
        red.set_low().ok();
        blue.set_low().ok();
    }
}
