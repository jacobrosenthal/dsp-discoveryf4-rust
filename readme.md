# Digital Signal Processing using Arm Cortex-M based Microcontrollers

## Requires

* Rust
* `rustup target add thumbv7em-none-eabihf`
* openocd from source
* `apt-get install binutils-arm-none-eabi`
* STM32F407G-DISC1 board
* Possibly updated stlink firmware
* udev rules /etc/udev/rules.d/49-stinkv2-1.rules and a reboot

```bash
SUBSYSTEMS=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374a", \
    MODE:="0666", \
    SYMLINK+="stlinkv2-1_%n"

SUBSYSTEMS=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", \
    MODE:="0666", \
    SYMLINK+="stlinkv2-1_%n"
```

## GDB debugging

For desperate cases, swap your runner in .cargo/config for the openocd configuration, start an open ocd server with `openocd -f interface/stlink-v2-1.cfg -f target/stm32f4x.cfg`, and `cargo run`.
