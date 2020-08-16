#![no_std]

use rtt_target::rprintln;

// if an panic happens, print it out and signal probe-run to exit
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rprintln!("{}", info);
    loop {
        cortex_m::asm::bkpt() // halt = exit probe-run
    }
}
