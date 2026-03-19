mod volcano;
mod venty;
mod crafty;

pub use volcano::VolcanoHybrid;
pub use venty::Venty;
pub use crafty::Crafty;

use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;

use crate::device::{DeviceModel, DeviceState};
use crate::error::StorzError;

/// Trait for controlling a Storz & Bickel vaporizer over BLE.
#[async_trait]
pub trait VaporizerControl: Send + Sync {
    /// Read the current measured temperature in Celsius.
    async fn get_current_temperature(&self) -> Result<f32, StorzError>;

    /// Read the target temperature in Celsius.
    async fn get_target_temperature(&self) -> Result<f32, StorzError>;

    /// Set the target temperature in Celsius.
    async fn set_target_temperature(&self, celsius: f32) -> Result<(), StorzError>;

    /// Turn the heater on.
    async fn heater_on(&self) -> Result<(), StorzError>;

    /// Turn the heater off.
    async fn heater_off(&self) -> Result<(), StorzError>;

    /// Turn the pump on (Volcano only, returns `UnsupportedOperation` for others).
    async fn pump_on(&self) -> Result<(), StorzError>;

    /// Turn the pump off (Volcano only, returns `UnsupportedOperation` for others).
    async fn pump_off(&self) -> Result<(), StorzError>;

    /// Read the current device state.
    async fn get_state(&self) -> Result<DeviceState, StorzError>;

    /// Subscribe to a stream of device state updates.
    async fn subscribe_state(
        &self,
    ) -> Result<Pin<Box<dyn Stream<Item = DeviceState> + Send>>, StorzError>;

    /// Return the device model.
    fn device_model(&self) -> DeviceModel;
}
