# This is a work in progress Rust Bare-Metal suite for a Raspberry Pi Zero 2 W

## What currently works

* GPIO (Input/Output only, only tested Output for pin 29)
* Timers using the system timer
* Displaying text (Only capital letters and spaces)

### Display Example

![title](Images/hello_world.png)

### How to Run on a Pi

First build the elf executable without debug symbols
```sh
cargo build --release
```
Then copy from the elf, copy to a flat binary named kernel7/kernel8 depending on how your Pi expects it for boot
```sh
aarch64-elf-objcopy -O binary -S target/aarch64-unknown-none/release/blinky ./kernel7.img
```
After that, just copy the kernel7.img file onto a FAT32 formatted sd card, that contains all of the boot firmware from the latest release of the [Raspberry Pi firmware repo](https://github.com/raspberrypi/firmware/releases)



##

I have no intention of making this a full library, it is only intended for learning and specific projects. However, I think some of the information may end up being useful for others so I will update it 