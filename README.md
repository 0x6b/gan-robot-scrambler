# gan-robot-scrambler

Scramble the [GAN Cube Robot](https://www.gancube.com/products/gan-speed-cube-robot) using the M5Stamp C3.

> [!CAUTION]
> Running this code may break your cube or robot. Use at your own risk.

## Tested Environment

- [M5Stamp C3 Mate with Pin Headers](https://shop.m5stack.com/products/m5stamp-c3-mate-with-pin-headers) (ESP32-C3)
- macOS Sequoia 15.2 (24C101)
- rustc nightly-2024-11-28

## Setup Development Environment

```console
$ brew install cmake ninja dfu-util
$ cargo install ldproxy
$ cargo install espflash
```

## Usage

Connect the M5Stamp C3 to your computer and run the following command:

```console
$ cp xtask-config.sample.json xtask-config.json
$ $EDITOR xtask-config.json # Edit the `env.ESPFLASH_PORT` field to match your setup
$ cargo x run
```

which will compile the code and flash it to the M5Stamp C3. When the LED is in blue, the M5Stamp C3 is ready; just click the button to scramble the cube.

The `xtask` along with `xtask-config.json` is just a thin wrapper around `cargo` and `espflash` that populates the environment variables from the `xtask-config.json` file.  You can also set the environment variables directly in your shell i.e.:

```console
$ ESPFLASH_PORT=/dev/cu.usbserial-xxxxxxxxxxx espflash flash \
    --monitor target/riscv32imc-esp-espidf/debug/gan-robot-scrambler
```

## References

- [The Rust on ESP Book](https://docs.esp-rs.org/book/installation/riscv.html)

## License

MIT. See [LICENSE](LICENSE) for details.
