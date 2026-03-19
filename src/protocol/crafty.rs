use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;
use btleplug::api::{Characteristic, Peripheral as _, WriteType};
use btleplug::platform::Peripheral;
use futures::{Stream, StreamExt};
use tokio::sync::{broadcast, Mutex};
use tokio_stream::wrappers::BroadcastStream;
use tracing::debug;

use crate::device::{DeviceModel, DeviceState};
use crate::error::StorzError;
use crate::protocol::VaporizerControl;
use crate::uuids::*;
use crate::utils;

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

    pub(crate) fn handle_notification(&self, uuid: uuid::Uuid, data: &[u8]) {
        let mut state = match self.state.try_lock() {
            Ok(s) => s,
            Err(_) => return,
        };

        match uuid {
            CRAFTY_CURRENT_TEMP_CHANGED => {
                if let Ok(temp) = utils::raw_to_celsius_u16(data) {
                    state.current_temp = Some(temp);
                    let _ = self.state_tx.send(state.clone());
                }
            }
            _ => {}
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
        Ok(Box::pin(BroadcastStream::new(rx).filter_map(|r| async move { r.ok() })))
    }

    fn device_model(&self) -> DeviceModel {
        DeviceModel::Crafty
    }
}
