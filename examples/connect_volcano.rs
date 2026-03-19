use std::time::Duration;

use storz_rs::{connect, discover_vaporizers, get_adapter};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let adapter = get_adapter().await?;
    let peripherals = discover_vaporizers(&adapter, Duration::from_secs(10)).await?;

    let volcano = peripherals
        .into_iter()
        .next()
        .expect("No Storz & Bickel device found");

    let device = connect(volcano).await?;
    info!("Connected to {}", device.device_model());

    let current = device.get_current_temperature().await?;
    info!("Current temperature: {current}°C");

    let target = device.get_target_temperature().await?;
    info!("Target temperature: {target}°C");

    info!("Setting target temperature to 185°C…");
    device.set_target_temperature(185.0).await?;

    info!("Turning heater ON");
    device.heater_on().await?;

    info!("Waiting 30 seconds…");
    tokio::time::sleep(Duration::from_secs(30)).await;

    info!("Turning heater OFF");
    device.heater_off().await?;

    info!("Done.");
    Ok(())
}
