# Scpi server

This is a replacement SCPI server for redpitaya.

## Building

```
rustup target add armv7-unknown-linux-gnueabihf
cargo build --target armv7-unknown-linux-gnueabihf --release
```

## Running

```
LD_LIBRARY_PATH=/opt/redpitaya/lib/ ./scpi-server
```
