use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;
use btleplug::api::{Characteristic, Peripheral as _, WriteType};
use btleplug::platform::Peripheral;
use futures::{Stream, StreamExt};
use tokio::sync::{Mutex, broadcast};
use tokio_stream::wrappers::BroadcastStream;
use tracing::{debug, warn};

use crate::device::{DeviceModel, DeviceState};
use crate::error::StorzError;
use crate::protocol::VaporizerControl;
use crate::utils;
use crate::uuids::*;

pub struct Crafty {
    peripheral: Peripheral,
    state: Arc<Mutex<DeviceState>>,
    state_tx: broadcast::Sender<DeviceState>,
}

impl Crafty {
    pub async fn new(peripheral: Peripheral) -> Result<Self, StorzError> {
        let (state_tx, _) = broadcast::channel(16);

        let device = Self {
            peripheral,
            state: Arc::new(Mutex::new(DeviceState::default())),
            state_tx,
        };

        device.init_notifications().await?;
        device.spawn_notification_loop();
        Ok(device)
    }

    async fn characteristic(&self, uuid: uuid::Uuid) -> Result<Characteristic, StorzError> {
        self.peripheral
            .characteristics()
            .into_iter()
            .find(|c| c.uuid == uuid)
            .ok_or_else(|| StorzError::ParseError(format!("Characteristic {uuid} not found")))
    }

    async fn init_notifications(&self) -> Result<(), StorzError> {
        let ch = self.characteristic(CRAFTY_CURRENT_TEMP_CHANGED).await?;
        self.peripheral.subscribe(&ch).await?;
        debug!("Subscribed to Crafty current temp notifications");
        Ok(())
    }

    fn spawn_notification_loop(&self) {
        let peripheral = self.peripheral.clone();
        let state = self.state.clone();
        let state_tx = self.state_tx.clone();

        tokio::spawn(async move {
            let mut stream = match peripheral.notifications().await {
                Ok(s) => s,
                Err(e) => {
                    warn!("Crafty failed to start notification stream: {e}");
                    return;
                }
            };

            while let Some(data) = stream.next().await {
                debug!(
                    "Crafty raw notification uuid={} bytes={:02X?}",
                    data.uuid, data.value
                );
                Self::handle_notification_inner(&state, &state_tx, data.uuid, &data.value);
            }

            warn!("Crafty notification stream ended (disconnect?)");
        });
    }

    fn handle_notification_inner(
        state: &Arc<Mutex<DeviceState>>,
        state_tx: &broadcast::Sender<DeviceState>,
        uuid: uuid::Uuid,
        data: &[u8],
    ) {
        let mut state = match state.try_lock() {
            Ok(s) => s,
            Err(_) => return,
        };

        if uuid == CRAFTY_CURRENT_TEMP_CHANGED {
            if let Ok(temp) = utils::raw_to_celsius_u16(data) {
                state.current_temp = Some(temp);
                let _ = state_tx.send(state.clone());
            }
        } else {
            debug!(
                "Crafty unhandled notification uuid={} len={}",
                uuid,
                data.len()
            );
        }
    }
}

#[async_trait]
impl VaporizerControl for Crafty {
    async fn get_current_temperature(&self) -> Result<f32, StorzError> {
        let ch = self.characteristic(CRAFTY_CURRENT_TEMP_CHANGED).await?;
        let data = self.peripheral.read(&ch).await?;
        utils::raw_to_celsius_u16(&data)
    }

    async fn get_target_temperature(&self) -> Result<f32, StorzError> {
        let ch = self.characteristic(CRAFTY_WRITE_TEMP).await?;
        let data = self.peripheral.read(&ch).await?;
        utils::raw_to_celsius_u16(&data)
    }

    async fn set_target_temperature(&self, celsius: f32) -> Result<(), StorzError> {
        let ch = self.characteristic(CRAFTY_WRITE_TEMP).await?;
        let raw = utils::celsius_to_raw_u16(celsius)?;
        self.peripheral
            .write(&ch, &raw, WriteType::WithoutResponse)
            .await?;
        debug!("Crafty target temp set to {celsius}°C");
        Ok(())
    }

    async fn heater_on(&self) -> Result<(), StorzError> {
        let ch = self.characteristic(CRAFTY_HEATER_ON).await?;
        self.peripheral
            .write(&ch, &[0x00], WriteType::WithoutResponse)
            .await?;
        debug!("Crafty heater ON");
        Ok(())
    }

    async fn heater_off(&self) -> Result<(), StorzError> {
        let ch = self.characteristic(CRAFTY_HEATER_OFF).await?;
        self.peripheral
            .write(&ch, &[0x00], WriteType::WithoutResponse)
            .await?;
        debug!("Crafty heater OFF");
        Ok(())
    }

    async fn pump_on(&self) -> Result<(), StorzError> {
        Err(StorzError::UnsupportedOperation {
            device: "Crafty".into(),
            operation: "pump_on".into(),
        })
    }

    async fn pump_off(&self) -> Result<(), StorzError> {
        Err(StorzError::UnsupportedOperation {
            device: "Crafty".into(),
            operation: "pump_off".into(),
        })
    }

    async fn get_state(&self) -> Result<DeviceState, StorzError> {
        let state = self.state.lock().await;
        Ok(state.clone())
    }

    async fn subscribe_state(
        &self,
    ) -> Result<Pin<Box<dyn Stream<Item = DeviceState> + Send>>, StorzError> {
        let rx = self.state_tx.subscribe();
        Ok(Box::pin(
            BroadcastStream::new(rx).filter_map(|r| async move { r.ok() }),
        ))
    }

    fn device_model(&self) -> DeviceModel {
        DeviceModel::Crafty
    }
}
