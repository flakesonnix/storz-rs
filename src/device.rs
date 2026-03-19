use std::fmt;

/// Supported Storz & Bickel device models.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceModel {
    VolcanoHybrid,
    Venty,
    Veazy,
    Crafty,
}

impl fmt::Display for DeviceModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceModel::VolcanoHybrid => write!(f, "Volcano Hybrid"),
            DeviceModel::Venty => write!(f, "Venty"),
            DeviceModel::Veazy => write!(f, "Veazy"),
            DeviceModel::Crafty => write!(f, "Crafty"),
        }
    }
}

/// Current state snapshot of a vaporizer device.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DeviceState {
    pub current_temp: Option<f32>,
    pub target_temp: Option<f32>,
    pub heater_on: bool,
    pub pump_on: bool,
    pub fan_on: bool,
    pub setpoint_reached: bool,
    pub raw_activity: Option<u32>,
    pub settings: Option<DeviceSettings>,
}

impl fmt::Display for DeviceState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "State(heater={}, pump={}, fan={}, current={:?}°C, target={:?}°C)",
            self.heater_on, self.pump_on, self.fan_on, self.current_temp, self.target_temp
        )
    }
}

/// Device settings (Venty/Veazy).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DeviceSettings {
    /// Temperature unit: true = Celsius, false = Fahrenheit
    pub is_celsius: bool,
    /// Setpoint reached flag
    pub setpoint_reached: bool,
    /// Boost visualization enabled
    pub boost_visualization: bool,
    /// Charge current optimization
    pub charge_current_optimization: bool,
    /// Charge voltage limit enabled
    pub charge_voltage_limit: bool,
    /// Permanent Bluetooth enabled
    pub permanent_bluetooth: bool,
    /// Auto shutdown timer in seconds
    pub auto_shutdown_seconds: Option<u16>,
    /// Battery level 0-100
    pub battery_level: Option<u8>,
    /// Is charging
    pub is_charging: bool,
}

/// Bitmask flags from the Volcano activity characteristic.
pub mod volcano_flags {
    pub const HEATER_ENABLED: u16 = 0x0020;
    pub const FAN_ENABLED: u16 = 0x0400;
    pub const AUTO_SHUTDOWN: u16 = 0x0200;
    pub const PUMP_ENABLED: u16 = 0x2000;
}
