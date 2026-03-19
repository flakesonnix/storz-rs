# storz-rs

**Control your Storz & Bickel vaporizers from Rust via Bluetooth Low Energy.**

[![crates.io](https://img.shields.io/crates/v/storz-rs)](https://crates.io/crates/storz-rs)
[![docs.rs](https://img.shields.io/docsrs/storz-rs)](https://docs.rs/storz-rs)
[![CI](https://github.com/storz-rs/storz-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/storz-rs/storz-rs/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A Rust library for controlling Storz & Bickel vaporizers over BLE. Built on [btleplug](https://github.com/deviceplug/btleplug) for cross-platform support. Protocol reverse-engineered from [reactive-volcano-app](https://github.com/firsttris/reactive-volcano-app).

![Storz & Bickel devices](docs/devices.png)

## Supported Devices

| Device | Tested | Notes |
|--------|--------|-------|
| Venty | ✅ | Temp, heater, auto init sequence |
| Volcano Hybrid | 🔬 | Temp, heater, pump, activity stream |
| Veazy | 🔬 | Same protocol as Venty |
| Crafty+ | 🔬 | Temp, heater on/off |

> Only the Venty has been tested so far. The others should work based on the shared protocol but haven't been verified with real hardware yet. If you have one and want to help test, open an issue.

## Quick Start

```rust
use std::time::Duration;
use storz_rs::{connect, discover_vaporizers, get_adapter};

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

Each example scans for nearby devices and connects to the first one it finds. `connect_venty` and `connect_volcano` are device-specific, `monitor_state` works with any device.

## API

Full docs at [docs.rs/storz-rs](https://docs.rs/storz-rs).

- `discover_vaporizers()` — BLE scan for S&B devices
- `connect()` — auto-detect model, connect, run init sequence
- `VaporizerControl` trait — uniform async API across all devices

| Method | Volcano | Venty | Veazy | Crafty |
|--------|---------|-------|-------|--------|
| `get_current_temperature` | ✅ | ✅ | ✅ | ✅ |
| `get/set_target_temperature` | ✅ | ✅ | ✅ | ✅ |
| `heater_on/off` | ✅ | ✅ | ✅ | ✅ |
| `pump_on/off` | ✅ | ❌ | ❌ | ❌ |
| `subscribe_state` | ✅ | ✅ | ✅ | ✅ |

`pump_on/off` returns `UnsupportedOperation` on devices without a pump.

## Platform Support

| Platform | BLE Backend | Status |
|----------|-------------|--------|
| Linux | BlueZ | ✅ |
| macOS | CoreBluetooth | ✅ |
| Windows | WinRT | ✅ |

Requires a BLE adapter.

## Troubleshooting

<details>
<summary>Linux: "No discovery started" / D-Bus error</summary>

BlueZ needs permissions to start BLE scans.

**Arch Linux** — no `bluetooth` group, use polkit:

```bash
sudo tee /etc/polkit-1/rules.d/50-bluetooth.rules << 'EOF'
polkit.addRule(function(action, subject) {
    if (action.id === "org.bluez.Adapter.StartDiscovery" ||
        action.id === "org.bluez.Adapter.SetDiscoveryFilter") {
        return polkit.Result.YES;
    }
});
EOF
```

**Debian/Ubuntu/Fedora** — add yourself to the `bluetooth` group:

```bash
sudo usermod -aG bluetooth $USER
```

Log out and back in after.

</details>

<details>
<summary>Linux: Bluetooth adapter not found</summary>

```bash
rfkill unblock bluetooth
bluetoothctl power on
systemctl status bluetooth
```

</details>

## Contributing

Fork, branch, PR. Run before submitting:

```bash
cargo fmt && cargo clippy -- -D warnings
```

Especially interested in testing on real hardware for Volcano Hybrid, Veazy, and Crafty+.

## Disclaimer

Not affiliated with Storz & Bickel GmbH. Use at your own risk. This is unofficial reverse-engineered software.

## License

MIT — see [LICENSE](LICENSE).
