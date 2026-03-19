# storz-rs

**Control your Storz & Bickel vaporizers from Rust via Bluetooth Low Energy.**

[![crates.io](https://img.shields.io/crates/v/storz-rs)](https://crates.io/crates/storz-rs)
[![docs.rs](https://img.shields.io/docsrs/storz-rs)](https://docs.rs/storz-rs)
[![CI](https://github.com/storz-rs/storz-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/storz-rs/storz-rs/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Scratching my own itch — I wanted to control my Storz & Bickel devices from the terminal without a phone app. Built on [btleplug](https://github.com/deviceplug/btleplug) for cross-platform BLE.

## Supported Devices

| Device | Status | Notes |
|--------|--------|-------|
| Volcano Hybrid | ✅ Tested | Full control: temp, heater, pump, activity stream |
| Venty | ✅ Tested | Temp control, heater, auto init sequence |
| Veazy | 🔬 Should work | Same protocol as Venty |
| Crafty+ | 🔬 Should work | Temp control, heater on/off |

## Quick Start

```rust
use std::time::Duration;
use storz_rs::{connect, discover_vaporizers, get_adapter, VaporizerControl};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let adapter = get_adapter().await?;
    let peripherals = discover_vaporizers(&adapter, Duration::from_secs(10)).await?;

    let device = connect(peripherals.into_iter().next().unwrap()).await?;

    device.set_target_temperature(185.0).await?;
    device.heater_on().await?;

    println!("Current temp: {}°C", device.get_current_temperature().await?);
    Ok(())
}
```

## Installation

```toml
[dependencies]
storz-rs = "0.1"
```

## Examples

```bash
cargo run --example connect_venty
cargo run --example connect_volcano
cargo run --example monitor_state
```

## API Overview

Full docs at [docs.rs/storz-rs](https://docs.rs/storz-rs).

Key types:
- `discover_vaporizers()` — BLE scan for S&B devices
- `connect()` — auto-detect model, connect, init
- `VaporizerControl` trait — `get/set_temperature`, `heater_on/off`, `pump_on/off`, `subscribe_state`

## Platform Support

| Platform | BLE Backend | Status |
|----------|-------------|--------|
| Linux | BlueZ | ✅ |
| macOS | CoreBluetooth | ✅ |
| Windows | WinRT | ✅/⚠️ |

Requires a BLE adapter. On Linux you may need `bluetoothctl` paired first.

## Contributing

Fork, branch, PR. Run before submitting:

```bash
cargo fmt && cargo clippy -- -D warnings
```

## Disclaimer

Not affiliated with Storz & Bickel GmbH. Use at your own risk. This is unofficial reverse-engineered software for personal use.

## License

MIT — see [LICENSE](LICENSE).
