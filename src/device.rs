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
    pub raw_activity: Option<u32>,
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

/// Bitmask flags from the Volcano activity characteristic.
pub mod volcano_flags {
    pub const HEATER_ENABLED: u16 = 0x0020;
    pub const AUTO_SHUTDOWN: u16 = 0x0200;
    pub const PUMP_ENABLED: u16 = 0x2000;
}
