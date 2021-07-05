# Scpi server

[![pipeline status](https://gitlab.com/yellow-pitaya/scpi-server/badges/main/pipeline.svg)](https://gitlab.com/yellow-pitaya/scpi-server/-/commits/main)
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
$ echo 'ECO:VERSION?' | netcat rp-xxxx.local 5000
0.97-489-ef96127 (ef96127)
```

## Debug

If you have a problem, you can display more messages:

```
# LD_LIBRARY_PATH=/opt/redpitaya/lib/ RUST_LOG=debug ./scpi-server
DEBUG 2018-02-24T14:12:44Z: scpi_server::server: Server started
DEBUG 2018-02-24T14:12:46Z: scpi_server::server: New client
DEBUG 2018-02-24T14:12:46Z: scpi_server::server: > "ECO:VERSION?\n"
 INFO 2018-02-24T14:12:46Z: scpi_server::server: Scpi(Version) []
DEBUG 2018-02-24T14:12:46Z: scpi_server::server: < 0.97-489-ef96127 (ef96127)
DEBUG 2018-02-24T14:12:46Z: scpi_server::server: Client served
```

## Install

See this [ansible
recipe](https://github.com/yellow-pitaya/controller/blob/master/redpitaya/tasks/scpi.yml).
