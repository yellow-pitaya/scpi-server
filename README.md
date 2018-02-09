# Scpi server

This is a replacement SCPI server for redpitaya.

## Building

```
rustup target add armv7-unknown-linux-gnueabihf
cargo build --target armv7-unknown-linux-gnueabihf --release
```

## Install

See this [ansible
recipe](https://github.com/yellow-pitaya/controller/blob/master/redpitaya/tasks/scpi.yml).
