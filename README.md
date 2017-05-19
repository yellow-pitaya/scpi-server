# Scpi server

This is a replacement SCPI server for redpitaya.

## Building

```
rustup target add armv7-unknown-linux-gnueabihf
cargo build --target armv7-unknown-linux-gnueabihf --release
```

## Install

```
ssh redpitaya 'mkdir --parent /opt/yellow-pitaya/bin'
scp target/armv7-unknown-linux-gnueabihf/release/scpi-server redpitaya:/opt/yellow-pitaya/bin
scp etc/yellow_pitaya_scpi.service redpitaya:/etc/systemd/system/
```

On the redpitaya, enable the service:

```
systemctl enable yellow_pitaya_scpi
systemctl start yellow_pitaya_scpi
```
