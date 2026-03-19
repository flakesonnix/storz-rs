use std::time::Duration;

use futures::StreamExt;
use storz_rs::{connect, discover_vaporizers, get_adapter, select_peripheral};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let adapter = get_adapter().await?;
    let peripherals = discover_vaporizers(&adapter, Duration::from_secs(10)).await?;
    let peripheral = select_peripheral(peripherals).await?;

    let device = connect(peripheral).await?;
    info!("Connected to {}", device.device_model());

    let mut stream = device.subscribe_state().await?;
    info!("Subscribed to state updates for 60 seconds…");

    let mut interval = tokio::time::interval(Duration::from_secs(5));
    let deadline = tokio::time::sleep(Duration::from_secs(60));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                info!("Monitoring complete.");
                break;
            }
            _ = interval.tick() => {
                if let Ok(state) = device.get_state().await {
                    info!("{state}");
                }
            }
            Some(state) = stream.next() => {
                info!("State update: {state}");
            }
        }
    }

    Ok(())
}
