mod crafty;
#[cfg(test)]
mod dummy;
mod venty;
mod volcano;

pub use crafty::Crafty;
pub use venty::Venty;
pub use volcano::VolcanoHybrid;

use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;

use crate::device::{DeviceInfo, DeviceModel, DeviceSettings, DeviceState};
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

    /// Read device settings.
    async fn get_settings(&self) -> Result<DeviceSettings, StorzError> {
        Err(StorzError::UnsupportedOperation {
            device: "unknown".into(),
            operation: "get_settings".into(),
        })
    }

    /// Set temperature unit (true = Celsius, false = Fahrenheit).
    async fn set_temperature_unit(&self, _celsius: bool) -> Result<(), StorzError> {
        Err(StorzError::UnsupportedOperation {
            device: "unknown".into(),
            operation: "set_temperature_unit".into(),
        })
    }

    /// Set the boost temperature offset in Celsius (Venty/Veazy/Crafty).
    async fn set_boost_temperature(&self, _celsius: f32) -> Result<(), StorzError> {
        Err(StorzError::UnsupportedOperation {
            device: "unknown".into(),
            operation: "set_boost_temperature".into(),
        })
    }

    /// Set the super-boost temperature offset in Celsius (Venty/Veazy).
    async fn set_super_boost_temperature(&self, _celsius: f32) -> Result<(), StorzError> {
        Err(StorzError::UnsupportedOperation {
            device: "unknown".into(),
            operation: "set_super_boost_temperature".into(),
        })
    }

    /// Set the auto-shutdown timer in seconds (Venty/Veazy).
    async fn set_auto_shutdown_timer(&self, _seconds: u16) -> Result<(), StorzError> {
        Err(StorzError::UnsupportedOperation {
            device: "unknown".into(),
            operation: "set_auto_shutdown_timer".into(),
        })
    }

    /// Set LED brightness (Volcano: 0-100, Venty/Veazy: 0-255).
    async fn set_brightness(&self, _value: u16) -> Result<(), StorzError> {
        Err(StorzError::UnsupportedOperation {
            device: "unknown".into(),
            operation: "set_brightness".into(),
        })
    }

    /// Enable or disable vibration (Volcano/Venty/Veazy).
    async fn set_vibration(&self, _on: bool) -> Result<(), StorzError> {
        Err(StorzError::UnsupportedOperation {
            device: "unknown".into(),
            operation: "set_vibration".into(),
        })
    }

    /// Trigger a factory reset (Venty/Veazy).
    async fn factory_reset(&self) -> Result<(), StorzError> {
        Err(StorzError::UnsupportedOperation {
            device: "unknown".into(),
            operation: "factory_reset".into(),
        })
    }

    /// Set boost visualization (Venty/Veazy).
    async fn set_boost_visualization(&self, _enabled: bool) -> Result<(), StorzError> {
        Err(StorzError::UnsupportedOperation {
            device: "unknown".into(),
            operation: "set_boost_visualization".into(),
        })
    }

    /// Set charge current optimization (Venty/Veazy).
    async fn set_charge_current_optimization(&self, _enabled: bool) -> Result<(), StorzError> {
        Err(StorzError::UnsupportedOperation {
            device: "unknown".into(),
            operation: "set_charge_current_optimization".into(),
        })
    }

    /// Set charge voltage limit (Venty/Veazy).
    async fn set_charge_voltage_limit(&self, _enabled: bool) -> Result<(), StorzError> {
        Err(StorzError::UnsupportedOperation {
            device: "unknown".into(),
            operation: "set_charge_voltage_limit".into(),
        })
    }

    /// Set permanent Bluetooth (Venty/Veazy).
    async fn set_permanent_bluetooth(&self, _enabled: bool) -> Result<(), StorzError> {
        Err(StorzError::UnsupportedOperation {
            device: "unknown".into(),
            operation: "set_permanent_bluetooth".into(),
        })
    }

    /// Set heater mode (Venty/Veazy: 0=off, 1=normal, 2=boost, 3=superboost).
    async fn set_heater_mode(
        &self,
        _mode: crate::device::HeaterMode,
    ) -> Result<(), StorzError> {
        Err(StorzError::UnsupportedOperation {
            device: "unknown".into(),
            operation: "set_heater_mode".into(),
        })
    }

    /// Read device information (serial, firmware, etc.).
    async fn get_device_info(&self) -> Result<DeviceInfo, StorzError> {
        Err(StorzError::UnsupportedOperation {
            device: "unknown".into(),
            operation: "get_device_info".into(),
        })
    }

    /// Set the auto-shutoff time in seconds (Volcano).
    async fn set_shutoff_time(&self, _seconds: u16) -> Result<(), StorzError> {
        Err(StorzError::UnsupportedOperation {
            device: "unknown".into(),
            operation: "set_shutoff_time".into(),
        })
    }

    /// Trigger the "find my device" feature (Venty/Veazy).
    /// Makes the device vibrate or beep to locate it.
    async fn find_my_device(&self) -> Result<(), StorzError> {
        Err(StorzError::UnsupportedOperation {
            device: "unknown".into(),
            operation: "find_my_device".into(),
        })
    }

    /// Return the device model.
    fn device_model(&self) -> DeviceModel;
}
