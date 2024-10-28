Arduino-rust-stepper-motor-template
===================================
[`avr-hal`](https://github.com/Rahix/avr-hal#readme)

[`ravedude`](https://crates.io/crates/ravedude)

Rust project for the _Arduino Uno_.

## About the Project

1. This is example show usage of 28BYJ-48 5-wire unipolar stepper motor that runs on 5V.
2. Here Digital Pins 2,3,4,5 is used as the output to stepper motor driver.
3. To run,
   ```bash
   cagro run
   ```

## Build Instructions
1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware.

3. Run `cargo run` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.