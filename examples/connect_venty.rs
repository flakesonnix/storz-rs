use std::time::Duration;

use storz_rs::{connect, discover_vaporizers, get_adapter};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let adapter = get_adapter().await?;
    let peripherals = discover_vaporizers(&adapter, Duration::from_secs(10)).await?;

    let venty = peripherals
        .into_iter()
        .next()
        .expect("No Storz & Bickel device found");

    // connect() automatically runs the Venty init sequence (0x02, 0x1D, 0x01, 0x04)
    let device = connect(venty).await?;
    info!("Connected to {}", device.device_model());

    info!("Setting target temperature to 190°C…");
    device.set_target_temperature(190.0).await?;

    info!("Turning heater ON");
    device.heater_on().await?;

    info!("Waiting 30 seconds…");
    tokio::time::sleep(Duration::from_secs(30)).await;

    info!("Turning heater OFF");
    device.heater_off().await?;

    info!("Done.");
    Ok(())
}
