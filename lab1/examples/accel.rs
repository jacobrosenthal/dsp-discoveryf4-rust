//! Led Blinky Roulette example using the DWT peripheral for timing.
//!
//! Requires cargo flash
//! `cargo install cargo-flash`
//!
//! `cargo flash --example roulette --release --chip STM32F407VGTx --protocol swd`

#![no_std]
#![no_main]

use stm32f4xx_hal as hal;

use crate::hal::{dwt::DwtExt, prelude::*, stm32};
use cortex_m;
use cortex_m_rt::entry;
use panic_halt as _;

// LIS302DL
/* Set configuration of LIS302DL MEMS Accelerometer *********************/
// lis302dl_initstruct.Power_Mode = LIS302DL_LOWPOWERMODE_ACTIVE;
// lis302dl_initstruct.Output_DataRate = LIS302DL_DATARATE_100;
// lis302dl_initstruct.Axes_Enable = LIS302DL_XYZ_ENABLE;
// lis302dl_initstruct.Full_Scale = LIS302DL_FULLSCALE_2_3;
// lis302dl_initstruct.Self_Test = LIS302DL_SELFTEST_NORMAL;

// /* Configure MEMS: data rate, power mode, full scale, self test and axes */
// ctrl = (uint16_t) (lis302dl_initstruct.Output_DataRate | lis302dl_initstruct.Power_Mode | \
//                    lis302dl_initstruct.Full_Scale | lis302dl_initstruct.Self_Test | \
//                    lis302dl_initstruct.Axes_Enable);

// /* Configure the accelerometer main parameters */
// AcceleroDrv->Init(ctrl);

// /* MEMS High Pass Filter configuration */
// lis302dl_filter.HighPassFilter_Data_Selection = LIS302DL_FILTEREDDATASELECTION_OUTPUTREGISTER;
// lis302dl_filter.HighPassFilter_CutOff_Frequency = LIS302DL_HIGHPASSFILTER_LEVEL_1;
// lis302dl_filter.HighPassFilter_Interrupt = LIS302DL_HIGHPASSFILTERINTERRUPT_1_2;

// /* Configure MEMS high pass filter cut-off level, interrupt and data selection bits */
// ctrl = (uint8_t)(lis302dl_filter.HighPassFilter_Data_Selection | \
//                  lis302dl_filter.HighPassFilter_CutOff_Frequency | \
//                  lis302dl_filter.HighPassFilter_Interrupt);

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // Set up the system clock.
    let rcc = dp.RCC.constrain();

    // Clock configuration is critical for RNG to work properly; otherwise
    // RNG_SR CECS bit will constantly report an error (if RNG_CLK < HCLK/16)
    // here we pick a simple clock configuration that ensures the pll48clk,
    // from which RNG_CLK is derived, is about 48 MHz
    let clocks = rcc
        .cfgr
        .use_hse(8.mhz()) //discovery board has 8 MHz crystal for HSE
        .sysclk(128.mhz())
        .freeze();

    // Create a delay abstraction based on DWT cycle counter
    let dwt = cp.DWT.constrain(cp.DCB, clocks);
    let mut delay = dwt.delay();

    let gpiod = dp.GPIOD.split();
    let mut led1 = gpiod.pd12.into_push_pull_output();
    let mut led2 = gpiod.pd13.into_push_pull_output();
    let mut led3 = gpiod.pd14.into_push_pull_output();
    let mut led4 = gpiod.pd15.into_push_pull_output();

    loop {
        led1.set_high().unwrap();
        led2.set_low().unwrap();
        led3.set_low().unwrap();
        led4.set_low().unwrap();
        delay.delay_ms(333_u32);

        led1.set_low().unwrap();
        led2.set_high().unwrap();
        led3.set_low().unwrap();
        led4.set_low().unwrap();
        delay.delay_ms(333_u32);

        led1.set_low().unwrap();
        led2.set_low().unwrap();
        led3.set_high().unwrap();
        led4.set_low().unwrap();
        delay.delay_ms(333_u32);

        led1.set_low().unwrap();
        led2.set_low().unwrap();
        led3.set_low().unwrap();
        led4.set_high().unwrap();
        delay.delay_ms(333_u32);
    }
}
