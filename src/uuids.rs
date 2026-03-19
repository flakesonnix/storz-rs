use uuid::Uuid;

// ── Device name prefixes for BLE scanning ──────────────────────────────────

pub const DEVICE_NAME_PREFIXES: &[&str] = &[
    "STORZ&BICKEL",
    "Storz&Bickel",
    "S&B VOLCANO",
    "S&B VY", // Venty
    "S&B VZ", // Veazy
    "S&B CRAFTY",
];

// ── Volcano Hybrid Services ────────────────────────────────────────────────

pub const VOLCANO_SERVICE_STATE: Uuid = uuid::uuid!("10100000-5354-4f52-5a26-4249434b454c");
pub const VOLCANO_SERVICE_CONTROL: Uuid = uuid::uuid!("10110000-5354-4f52-5a26-4249434b454c");

// ── Volcano Hybrid Characteristics ─────────────────────────────────────────

pub const VOLCANO_CURRENT_TEMP: Uuid = uuid::uuid!("10110001-5354-4f52-5a26-4249434b454c");
pub const VOLCANO_TARGET_TEMP: Uuid = uuid::uuid!("10110003-5354-4f52-5a26-4249434b454c");
pub const VOLCANO_HEATER_ON: Uuid = uuid::uuid!("1011000f-5354-4f52-5a26-4249434b454c");
pub const VOLCANO_HEATER_OFF: Uuid = uuid::uuid!("10110010-5354-4f52-5a26-4249434b454c");
pub const VOLCANO_PUMP_ON: Uuid = uuid::uuid!("10110013-5354-4f52-5a26-4249434b454c");
pub const VOLCANO_PUMP_OFF: Uuid = uuid::uuid!("10110014-5354-4f52-5a26-4249434b454c");
pub const VOLCANO_ACTIVITY: Uuid = uuid::uuid!("1010000c-5354-4f52-5a26-4249434b454c");
pub const VOLCANO_SHUTOFF_TIME: Uuid = uuid::uuid!("1011000d-5354-4f52-5a26-4249434b454c");
pub const VOLCANO_BRIGHTNESS: Uuid = uuid::uuid!("10110005-5354-4f52-5a26-4249434b454c");
pub const VOLCANO_VIBRATION: Uuid = uuid::uuid!("1010000e-5354-4f52-5a26-4249434b454c");
pub const VOLCANO_SERIAL_NUMBER: Uuid = uuid::uuid!("10100008-5354-4f52-5a26-4249434b454c");
pub const VOLCANO_FIRMWARE_VERSION: Uuid = uuid::uuid!("10100003-5354-4f52-5a26-4249434b454c");
pub const VOLCANO_FIRMWARE_BLE_VERSION: Uuid = uuid::uuid!("10100004-5354-4f52-5a26-4249434b454c");
pub const VOLCANO_HOURS_OF_HEATING: Uuid = uuid::uuid!("10110015-5354-4f52-5a26-4249434b454c");
pub const VOLCANO_MINUTES_OF_HEATING: Uuid = uuid::uuid!("10110016-5354-4f52-5a26-4249434b454c");

// ── Venty / Veazy Services ─────────────────────────────────────────────────

pub const VENTY_SERVICE_PRIMARY: Uuid = uuid::uuid!("00000000-5354-4f52-5a26-4249434b454c");

// ── Venty / Veazy Characteristics ──────────────────────────────────────────

pub const VENTY_CONTROL: Uuid = uuid::uuid!("00000001-5354-4f52-5a26-4249434b454c");

// ── Crafty+ Services ──────────────────────────────────────────────────────

pub const CRAFTY_SERVICE_1: Uuid = uuid::uuid!("00000001-4c45-4b43-4942-265a524f5453");
pub const CRAFTY_SERVICE_2: Uuid = uuid::uuid!("00000002-4c45-4b43-4942-265a524f5453");
pub const CRAFTY_SERVICE_3: Uuid = uuid::uuid!("00000003-4c45-4b43-4942-265a524f5453");

// ── Crafty+ Characteristics ───────────────────────────────────────────────

pub const CRAFTY_WRITE_TEMP: Uuid = uuid::uuid!("00000021-4c45-4b43-4942-265a524f5453");
pub const CRAFTY_CURRENT_TEMP_CHANGED: Uuid = uuid::uuid!("00000011-4c45-4b43-4942-265a524f5453");
pub const CRAFTY_WRITE_BOOST_TEMP: Uuid = uuid::uuid!("00000031-4c45-4b43-4942-265a524f5453");
pub const CRAFTY_HEATER_ON: Uuid = uuid::uuid!("00000081-4c45-4b43-4942-265a524f5453");
pub const CRAFTY_HEATER_OFF: Uuid = uuid::uuid!("00000091-4c45-4b43-4942-265a524f5453");
pub const CRAFTY_FIRMWARE_VERSION: Uuid = uuid::uuid!("00000032-4c45-4b43-4942-265a524f5453");
pub const CRAFTY_FIRMWARE_BLE_VERSION: Uuid = uuid::uuid!("00000072-4c45-4b43-4942-265a524f5453");
pub const CRAFTY_SYSTEM_STATUS: Uuid = uuid::uuid!("00000083-4c45-4b43-4942-265a524f5453");
pub const CRAFTY_AKKU_STATUS: Uuid = uuid::uuid!("00000063-4c45-4b43-4942-265a524f5453");
pub const CRAFTY_AKKU_STATUS_2: Uuid = uuid::uuid!("00000073-4c45-4b43-4942-265a524f5453");
pub const CRAFTY_LED_BRIGHTNESS: Uuid = uuid::uuid!("00000051-4c45-4b43-4942-265a524f5453");
pub const CRAFTY_AUTO_OFF_COUNTDOWN: Uuid = uuid::uuid!("00000061-4c45-4b43-4942-265a524f5453");
pub const CRAFTY_AUTO_OFF_CURRENT: Uuid = uuid::uuid!("00000071-4c45-4b43-4942-265a524f5453");
pub const CRAFTY_POWER_CHANGED: Uuid = uuid::uuid!("00000041-4c45-4b43-4942-265a524f5453");
pub const CRAFTY_USE_HOURS: Uuid = uuid::uuid!("00000023-4c45-4b43-4942-265a524f5453");
pub const CRAFTY_USE_MINUTES: Uuid = uuid::uuid!("000001e3-4c45-4b43-4942-265a524f5453");
