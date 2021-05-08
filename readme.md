# Digital Signal Processing using Arm Cortex-M based Microcontrollers

Translating the book from C to Rust. No relation to the author.

## Requires

* Rust 1.5.1
* `rustup target add thumbv7em-none-eabihf`
* STM32F407G-DISC1 board
* Possibly updated stlink firmware
* (linux) udev rules /etc/udev/rules.d/49-stinkv2-1.rules and a reboot

```bash
SUBSYSTEMS=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374a", \
    MODE:="0666", \
    SYMLINK+="stlinkv2-1_%n"

SUBSYSTEMS=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", \
    MODE:="0666", \
    SYMLINK+="stlinkv2-1_%n"
```

## GDB debugging

Requires:

* OpenOCD 0.11.0

For desperate cases, swap your runner in .cargo/config for the openocd configuration, start an open ocd server with `openocd -f interface/stlink-v2-1.cfg -f target/stm32f4x.cfg`, and `cargo run`.
