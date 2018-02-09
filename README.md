# Scpi server

This is a replacement SCPI server for redpitaya.

## Building

To cross-compile scpi-server, you need `arm-linux-gnueabihf-gcc` and configure
the linker in `~/.cargo/config`:

```
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```

Then add the `armv7-unknown-linux-gnueabihf` target to rust toolchain:

```
rustup target add armv7-unknown-linux-gnueabihf
```

And finally:

```
cargo build --target armv7-unknown-linux-gnueabihf --release
```

## Install

See this [ansible
recipe](https://github.com/yellow-pitaya/controller/blob/master/redpitaya/tasks/scpi.yml).
