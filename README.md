# gan-robot-scrambler

Scramble the [GAN Cube Robot](https://www.gancube.com/products/gan-speed-cube-robot) using the M5Stamp C3.

> [!CAUTION]
> Running this code may break your cube or robot. Use at your own risk.

## Tested Environment

- [M5Stamp C3 Mate with Pin Headers](https://shop.m5stack.com/products/m5stamp-c3-mate-with-pin-headers) (ESP32-C3)
- [GAN Cube Robot](https://www.gancube.com/products/gan-speed-cube-robot)
- Rust 1.85.0-nightly (6b6a867ae 2024-11-27)
- macOS Sequoia 15.2 (24C101)

## Setup Development Environment

This project is `std` application. Make sure you have met the following requirements from the Rust on ESP book:

1. [Rust installation](https://docs.esp-rs.org/book/installation/rust.html)
2. [RISC-V targets only](https://docs.esp-rs.org/book/installation/riscv.html)

## Usage

Connect the M5Stamp C3 to your computer and run the following command:

```console
$ cp xtask-config.sample.json xtask-config.json
$ $EDITOR xtask-config.json # Edit the `env.ESPFLASH_PORT` field to match your setup
$ cargo x --help
Usage: cargo <x|xtask> [OPTIONS] <COMMAND>

Commands:
  run             Build and flash the program to the board
  build           Build the program
  clean           Remove the target directory
  serial-console  Open a serial console
  help            Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>  [default: xtask-config.json]
  -h, --help             Print help
```

`cargo x run` will compile the code and flash it to the M5Stamp C3. When the LED is in blue, the M5Stamp C3 is ready; just click the button to scramble the cube.

The `xtask` along with `xtask-config.json` is just a thin wrapper around `cargo` and `espflash` that populates the environment variables from the `xtask-config.json` file. You can also set the environment variables directly in your shell i.e.:

```console
$ ESPFLASH_PORT=/dev/cu.usbserial-xxxxxxxxxxx espflash flash \
    --monitor target/riscv32imc-esp-espidf/debug/gan-robot-scrambler
```

### Build troubleshooting

1. Run `cargo x clean --all` to remove all build artifacts and try again.
2. Check the `rust-toolchain.toml`, and update the nightly version (`toolchain.channel`) if necessary.
3. Update template with `cargo generate esp-rs/esp-idf-template cargo` to start fresh.

## References

- [The Rust on ESP Book](https://docs.esp-rs.org/book/)

## Acknowledgements

[cubing/cubing.js](https://github.com/cubing/cubing.js), especially the [GanRobot.ts](https://github.com/cubing/cubing.js/blob/19e893db4d6b2feaeafd4e40f3a5183b6bad88fc/src/cubing/bluetooth/smart-robot/GanRobot.ts), for the GAN robot control logic.

## License

MIT. See [LICENSE](LICENSE) for details.
