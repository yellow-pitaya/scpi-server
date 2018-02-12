# Scpi server

[![Build Status](https://travis-ci.org/yellow-pitaya/scpi-server.svg?branch=master)](https://travis-ci.org/yellow-pitaya/scpi-server)

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

## Testing

Start the server on your repitaya:

```
# cat /opt/redpitaya/fpga/fpga_0.94.bit > /dev/xdevcfg
# LD_LIBRARY_PATH=/opt/redpitaya/lib/ ./scpi-server
```

Then, on your computer:

```
$ echo '' | netcat rp-xxxx.local
0.97-489-ef96127 (ef96127)
```

## Install

See this [ansible
recipe](https://github.com/yellow-pitaya/controller/blob/master/redpitaya/tasks/scpi.yml).
