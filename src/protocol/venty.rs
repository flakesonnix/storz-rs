use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;
use btleplug::api::{Peripheral as _, WriteType};
use btleplug::platform::Peripheral;
use futures::Stream;
use futures::StreamExt;
use tokio::sync::{broadcast, Mutex};
use tokio_stream::wrappers::BroadcastStream;
use tracing::{debug, warn};

use crate::device::{DeviceModel, DeviceState};
use crate::error::StorzError;
use crate::protocol::VaporizerControl;
use crate::utils;
use crate::uuids::*;

/// Venty or Veazy device, both share the same protocol.
pub struct Venty {
    peripheral: Peripheral,
    model: DeviceModel,
    state: Arc<Mutex<DeviceState>>,
    state_tx: broadcast::Sender<DeviceState>,
}

impl Venty {
    pub async fn new(peripheral: Peripheral, model: DeviceModel) -> Result<Self, StorzError> {
        let (state_tx, _) = broadcast::channel(16);

        let device = Self {
            peripheral,
            model,
            state: Arc::new(Mutex::new(DeviceState::default())),
            state_tx,
        };

        device.init().await?;
        device.spawn_notification_loop();
        Ok(device)
    }

    async fn control_characteristic(&self) -> Result<btleplug::api::Characteristic, StorzError> {
        self.peripheral
            .characteristics()
            .into_iter()
            .find(|c| c.uuid == VENTY_CONTROL)
            .ok_or_else(|| StorzError::ParseError("Venty control characteristic not found".into()))
    }

    async fn write_command(&self, buf: &[u8]) -> Result<(), StorzError> {
        let ch = self.control_characteristic().await?;
        self.peripheral
            .write(&ch, buf, WriteType::WithoutResponse)
            .await?;
        Ok(())
    }

    async fn init(&self) -> Result<(), StorzError> {
        let ch = self.control_characteristic().await?;

        // Subscribe to notifications first
        self.peripheral.subscribe(&ch).await?;
        debug!("Subscribed to Venty/Veazy control notifications");

        // Send initialization sequence: 0x02, 0x1D, 0x01, 0x04
        for &cmd in &[0x02u8, 0x1Du8, 0x01u8, 0x04u8] {
            let buf = utils::build_venty_command(cmd, 0, &[]);
            self.write_command(&buf).await?;
            debug!("Venty/Veazy init command 0x{cmd:02X} sent");
        }

        Ok(())
    }

    fn spawn_notification_loop(&self) {
        let peripheral = self.peripheral.clone();
        let state = self.state.clone();
        let state_tx = self.state_tx.clone();
        let model = self.model;

        tokio::spawn(async move {
            let mut stream = match peripheral.notifications().await {
                Ok(s) => s,
                Err(e) => {
                    warn!("Venty/Veazy failed to start notification stream: {e}");
                    return;
                }
            };

            while let Some(data) = stream.next().await {
                debug!(
                    "Venty/Veazy raw notification uuid={} bytes={:02X?}",
                    data.uuid, data.value
                );
                Self::handle_notification_inner(&state, &state_tx, &data.value);
            }

            warn!("{model} notification stream ended (disconnect?)");
        });
    }

    fn handle_notification_inner(
        state: &Arc<Mutex<DeviceState>>,
        state_tx: &broadcast::Sender<DeviceState>,
        data: &[u8],
    ) {
        if data.is_empty() {
            return;
        }

        let mut state = match state.try_lock() {
            Ok(s) => s,
            Err(_) => return,
        };

        let cmd_id = data[0];

        match cmd_id {
            0x01 | 0x05 if data.len() >= 15 => {
                // Bytes 2+3: current temperature (u16 LE, /10)
                if data.len() >= 4 {
                    let raw = u16::from_le_bytes([data[2], data[3]]);
                    state.current_temp = Some((raw as f32) / 10.0);
                }

                // Bytes 4+5: target temperature (u16 LE, /10)
                if data.len() >= 6 {
                    let raw = u16::from_le_bytes([data[4], data[5]]);
                    state.target_temp = Some((raw as f32) / 10.0);
                }

                // Byte 11: heater mode (0=off, >0=on)
                if data.len() >= 12 {
                    state.heater_on = data[11] > 0;
                }

                // Venty/Veazy don't have pumps
                state.pump_on = false;

                let _ = state_tx.send(state.clone());
            }
            _ => {
                debug!(
                    "Venty/Veazy unhandled notification cmd=0x{:02X} len={}",
                    cmd_id,
                    data.len()
                );
            }
        }
    }
}

#[async_trait]
impl VaporizerControl for Venty {
    async fn get_current_temperature(&self) -> Result<f32, StorzError> {
        // Venty doesn't expose current temp directly via a read;
        // it comes through notifications. Return cached value.
        let state = self.state.lock().await;
        state.current_temp.ok_or(StorzError::ParseError(
            "Current temperature not yet available from device notifications".into(),
        ))
    }

    async fn get_target_temperature(&self) -> Result<f32, StorzError> {
        let state = self.state.lock().await;
        state.target_temp.ok_or(StorzError::ParseError(
            "Target temperature not yet available from device notifications".into(),
        ))
    }

    async fn set_target_temperature(&self, celsius: f32) -> Result<(), StorzError> {
        let raw = (celsius * 10.0).round() as u16;
        let low = (raw & 0xFF) as u8;
        let high = ((raw >> 8) & 0xFF) as u8;

        let buf = utils::build_venty_command(
            0x01, // Write command
            0x02, // SET_TEMPERATURE mask
            &[(4, low), (5, high)],
        );
        self.write_command(&buf).await?;
        debug!("Venty/Veazy target temp set to {celsius}°C");
        Ok(())
    }

    async fn heater_on(&self) -> Result<(), StorzError> {
        let buf = utils::build_venty_command(
            0x01,
            0x20,       // HEATER mask
            &[(11, 1)], // heater mode = 1 (normal)
        );
        self.write_command(&buf).await?;
        debug!("Venty/Veazy heater ON");
        Ok(())
    }

    async fn heater_off(&self) -> Result<(), StorzError> {
        let buf = utils::build_venty_command(
            0x01,
            0x20,       // HEATER mask
            &[(11, 0)], // heater mode = 0 (off)
        );
        self.write_command(&buf).await?;
        debug!("Venty/Veazy heater OFF");
        Ok(())
    }

    async fn pump_on(&self) -> Result<(), StorzError> {
        Err(StorzError::UnsupportedOperation {
            device: self.model.to_string(),
            operation: "pump_on".into(),
        })
    }

    async fn pump_off(&self) -> Result<(), StorzError> {
        Err(StorzError::UnsupportedOperation {
            device: self.model.to_string(),
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
        self.model
    }
}
