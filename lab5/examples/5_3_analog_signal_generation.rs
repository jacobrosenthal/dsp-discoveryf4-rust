//! This project is used for generating analog signal from DAC output.
//! Here, we first create two look-up table, sin_lookup and sq_lookup, for
//! signal generation. Then, the signal is generated by triggering DAC in timer
//! interrupt subroutine using these tables. Here, the signal waveform can be
//! changed by pressing onboard push button of the STM32F4 Discovery kit.  
//!
//! Requires cargo embed `cargo install cargo-embed`
//!
//! `cargo embed --example 5_3_analog_signal_generation`

#![no_std]
#![no_main]

use stm32f4xx_hal as hal;

use crate::hal::{
    dac::DacOut,
    dac::DacPin,
    gpio::{gpioa::PA0, Edge, ExtiPin, Input, PullDown},
    interrupt,
    prelude::*,
    stm32,
    timer::Timer,
};
use core::cell::RefCell;
use core::f32::consts::PI;
use core::ops::DerefMut;
use core::sync::atomic::{AtomicBool, Ordering};
use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;
use micromath::F32Ext;
use nb::block;
use panic_rtt as _;
use typenum::Unsigned;

type N = heapless::consts::U160;

static BUTTON: Mutex<RefCell<Option<PA0<Input<PullDown>>>>> = Mutex::new(RefCell::new(None));
static FLAG: AtomicBool = AtomicBool::new(true);

#[entry]
fn main() -> ! {
    let mut dp = stm32::Peripherals::take().unwrap();
    let _cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // Set up the system clock.
    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz()) //discovery board has 8 MHz crystal for HSE
        .sysclk(168.mhz())
        .freeze();

    let gpioa = dp.GPIOA.split();

    let mut board_btn = gpioa.pa0.into_pull_down_input();
    board_btn.make_interrupt_source(&mut dp.SYSCFG);
    board_btn.enable_interrupt(&mut dp.EXTI);
    board_btn.trigger_on_edge(&mut dp.EXTI, Edge::FALLING);

    free(|cs| {
        BUTTON.borrow(cs).replace(Some(board_btn));
    });

    // Enable interrupts
    stm32::NVIC::unpend(hal::stm32::Interrupt::EXTI0);
    unsafe {
        stm32::NVIC::unmask(hal::stm32::Interrupt::EXTI0);
    };

    let mut dac = dp.DAC.constrain(gpioa.pa4.into_analog());

    dac.enable();

    let sq_lookup = (0..N::to_usize())
        .map(|n| if n < N::to_usize() / 2 { 4095 } else { 0 })
        .collect::<heapless::Vec<u16, N>>();

    // period 160
    let sin_lookup = (0..N::to_usize())
        .map(|n| {
            let sindummy = (2.0 * PI * n as f32 / N::to_u16() as f32).sin();
            ((sindummy * 2047.0) + 2048.0) as u16
        })
        .collect::<heapless::Vec<u16, N>>();

    // frequency dac 16khz, freq/period = 16000/160 = 100hz
    let mut timer = Timer::tim1(dp.TIM1, 16.khz(), clocks);
    // im not sure if you can create and start twice?
    block!(timer.wait()).unwrap();

    loop {
        // little wiggly because not an interrupt..
        let sin = FLAG.load(Ordering::Relaxed);
        if sin {
            for n in 0..N::to_usize() {
                dac.set_value(sin_lookup[n]);
                timer.start(16.khz());
                block!(timer.wait()).unwrap();
            }
        } else {
            for n in 0..N::to_usize() {
                dac.set_value(sq_lookup[n]);
                timer.start(16.khz());
                block!(timer.wait()).unwrap();
            }
        }
    }
}

// todo what orderings?
// todo no cap, need debouncing
#[interrupt]
fn EXTI0() {
    free(|cs| {
        let mut btn_ref = BUTTON.borrow(cs).borrow_mut();
        if let Some(ref mut btn) = btn_ref.deref_mut() {
            btn.clear_interrupt_pending_bit();
        }
        let flag = FLAG.load(Ordering::Relaxed);
        FLAG.store(!flag, Ordering::Relaxed);
    });
}
