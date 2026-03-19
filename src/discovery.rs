use std::time::Duration;

use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use tokio::time;
use tracing::{debug, info, warn};

use crate::error::StorzError;
use crate::uuids::DEVICE_NAME_PREFIXES;

/// Scan for Storz & Bickel devices via BLE and return discovered peripherals.
///
/// The `timeout` controls how long to scan before returning results.
pub async fn discover_vaporizers(
    adapter: &Adapter,
    timeout: Duration,
) -> Result<Vec<Peripheral>, StorzError> {
    info!("Starting BLE scan for Storz & Bickel devices ({timeout:?})…");

    // Ensure adapter is powered on before scanning
    match adapter.adapter_info().await {
        Ok(info_str) => debug!("Adapter info: {info_str}"),
        Err(e) => warn!("Could not read adapter info: {e}"),
    }

    adapter
        .start_scan(ScanFilter::default())
        .await
        .map_err(|e| {
            StorzError::Bluetooth(btleplug::Error::Other(
                format!(
                    "Failed to start BLE scan: {e}\n\n\
                     Troubleshooting:\n\
                     1. Ensure Bluetooth is enabled: rfkill unblock bluetooth\n\
                     2. Ensure adapter is powered on: bluetoothctl power on\n\
                     3. Try running with elevated permissions (sudo or bluetooth group)\n\
                     4. Check: bluetoothctl show"
                )
                .into(),
            ))
        })?;

    time::sleep(timeout).await;
    adapter.stop_scan().await?;

    let peripherals = adapter.peripherals().await?;
    let mut found = Vec::new();

    for p in peripherals {
        if let Ok(Some(props)) = p.properties().await {
            if let Some(name) = props.local_name.as_ref() {
                if DEVICE_NAME_PREFIXES
                    .iter()
                    .any(|prefix| name.contains(prefix))
                {
                    info!("Found device: {name}");
                    found.push(p);
                }
            }
        }
    }

    if found.is_empty() {
        warn!("No Storz & Bickel devices found during scan");
    } else {
        debug!("Discovered {} device(s)", found.len());
    }

    Ok(found)
}

/// Obtain the default BLE adapter.
pub async fn get_adapter() -> Result<Adapter, StorzError> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    adapters
        .into_iter()
        .next()
        .ok_or(StorzError::DeviceNotFound)
}
