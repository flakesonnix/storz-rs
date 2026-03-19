use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;
use btleplug::api::{Characteristic, Peripheral as _, WriteType};
use btleplug::platform::Peripheral;
use futures::Stream;
use tokio::sync::{broadcast, Mutex};
use tokio_stream::wrappers::BroadcastStream;
use futures::StreamExt;
use tracing::debug;

use crate::device::{volcano_flags, DeviceModel, DeviceState};
use crate::error::StorzError;
use crate::protocol::VaporizerControl;
use crate::uuids::*;
use crate::utils;

pub struct VolcanoHybrid {
    peripheral: Peripheral,
    state: Arc<Mutex<DeviceState>>,
    state_tx: broadcast::Sender<DeviceState>,
}

impl VolcanoHybrid {
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

    async fn write_u8(&self, uuid: uuid::Uuid) -> Result<(), StorzError> {
        let ch = self.characteristic(uuid).await?;
        self.peripheral
            .write(&ch, &[0x00], WriteType::WithoutResponse)
            .await?;
        Ok(())
    }

    async fn init_notifications(&self) -> Result<(), StorzError> {
        let ch = self.characteristic(VOLCANO_ACTIVITY).await?;
        self.peripheral.subscribe(&ch).await?;
        debug!("Subscribed to Volcano activity notifications");

        let ch = self.characteristic(VOLCANO_CURRENT_TEMP).await?;
        self.peripheral.subscribe(&ch).await?;
        debug!("Subscribed to Volcano current temp notifications");

        let ch = self.characteristic(VOLCANO_TARGET_TEMP).await?;
        self.peripheral.subscribe(&ch).await?;
        debug!("Subscribed to Volcano target temp notifications");

        Ok(())
    }

    pub(crate) fn handle_notification(&self, uuid: uuid::Uuid, data: &[u8]) {
        let mut state = match self.state.try_lock() {
            Ok(s) => s,
            Err(_) => return,
        };

        match uuid {
            VOLCANO_ACTIVITY => {
                if data.len() >= 2 {
                    let flags = u16::from_le_bytes([data[0], data[1]]);
                    state.raw_activity = Some(flags as u32);
                    state.heater_on = (flags & volcano_flags::HEATER_ENABLED) != 0;
                    state.pump_on = (flags & volcano_flags::PUMP_ENABLED) != 0;
                    let _ = self.state_tx.send(state.clone());
                }
            }
            VOLCANO_CURRENT_TEMP => {
                if let Ok(temp) = utils::raw_to_celsius_u16(data) {
                    state.current_temp = Some(temp);
                    let _ = self.state_tx.send(state.clone());
                }
            }
            VOLCANO_TARGET_TEMP => {
                if let Ok(temp) = utils::raw_to_celsius_u32(data) {
                    state.target_temp = Some(temp);
                    let _ = self.state_tx.send(state.clone());
                }
            }
            _ => {}
        }
    }
}

#[async_trait]
impl VaporizerControl for VolcanoHybrid {
    async fn get_current_temperature(&self) -> Result<f32, StorzError> {
        let ch = self.characteristic(VOLCANO_CURRENT_TEMP).await?;
        let data = self.peripheral.read(&ch).await?;
        utils::raw_to_celsius_u16(&data)
    }

    async fn get_target_temperature(&self) -> Result<f32, StorzError> {
        let ch = self.characteristic(VOLCANO_TARGET_TEMP).await?;
        let data = self.peripheral.read(&ch).await?;
        utils::raw_to_celsius_u32(&data)
    }

    async fn set_target_temperature(&self, celsius: f32) -> Result<(), StorzError> {
        let ch = self.characteristic(VOLCANO_TARGET_TEMP).await?;
        let raw = utils::celsius_to_raw_u32(celsius)?;
        self.peripheral
            .write(&ch, &raw, WriteType::WithoutResponse)
            .await?;
        debug!("Volcano target temp set to {celsius}°C");
        Ok(())
    }

    async fn heater_on(&self) -> Result<(), StorzError> {
        self.write_u8(VOLCANO_HEATER_ON).await?;
        debug!("Volcano heater ON");
        Ok(())
    }

    async fn heater_off(&self) -> Result<(), StorzError> {
        self.write_u8(VOLCANO_HEATER_OFF).await?;
        debug!("Volcano heater OFF");
        Ok(())
    }

    async fn pump_on(&self) -> Result<(), StorzError> {
        self.write_u8(VOLCANO_PUMP_ON).await?;
        debug!("Volcano pump ON");
        Ok(())
    }

    async fn pump_off(&self) -> Result<(), StorzError> {
        self.write_u8(VOLCANO_PUMP_OFF).await?;
        debug!("Volcano pump OFF");
        Ok(())
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
        DeviceModel::VolcanoHybrid
    }
}
