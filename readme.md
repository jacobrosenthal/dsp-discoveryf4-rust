# Digital Signal Processing using Arm Cortex-M based Microcontrollers

Requires:

* Rust
* `rustup target add thumbv7em-none-eabihf`
* openocd from source
* `apt-get install binutils-arm-none-eabi`
* STM32F407G-DISC1 board
* Possibly updated stlink firmware
* udev rules /etc/udev/rules.d/49-stinkv2-1.rules and a reboot
```
SUBSYSTEMS=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374a", \
    MODE:="0666", \
    SYMLINK+="stlinkv2-1_%n"

SUBSYSTEMS=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", \
    MODE:="0666", \
    SYMLINK+="stlinkv2-1_%n"
```
