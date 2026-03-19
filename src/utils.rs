use crate::error::StorzError;

/// Minimum valid temperature in Celsius.
pub const TEMP_MIN: f32 = 40.0;
/// Maximum valid temperature in Celsius.
pub const TEMP_MAX: f32 = 230.0;

/// Convert Celsius to raw BLE bytes (u32 little-endian, value × 10).
pub fn celsius_to_raw_u32(celsius: f32) -> Result<[u8; 4], StorzError> {
    if !(TEMP_MIN..=TEMP_MAX).contains(&celsius) {
        return Err(StorzError::ParseError(format!(
            "Temperature {celsius}°C out of range ({TEMP_MIN}–{TEMP_MAX}°C)"
        )));
    }
    let raw = (celsius * 10.0).round() as u32;
    Ok(raw.to_le_bytes())
}

/// Convert Celsius to raw BLE bytes (u16 little-endian, value × 10).
pub fn celsius_to_raw_u16(celsius: f32) -> Result<[u8; 2], StorzError> {
    if !(TEMP_MIN..=TEMP_MAX).contains(&celsius) {
        return Err(StorzError::ParseError(format!(
            "Temperature {celsius}°C out of range ({TEMP_MIN}–{TEMP_MAX}°C)"
        )));
    }
    let raw = (celsius * 10.0).round() as u16;
    Ok(raw.to_le_bytes())
}

/// Parse a u16 little-endian BLE value to Celsius (value ÷ 10).
pub fn raw_to_celsius_u16(bytes: &[u8]) -> Result<f32, StorzError> {
    if bytes.len() < 2 {
        return Err(StorzError::ParseError(format!(
            "Expected >= 2 bytes for u16, got {}",
            bytes.len()
        )));
    }
    let raw = u16::from_le_bytes([bytes[0], bytes[1]]);
    Ok((raw as f32) / 10.0)
}

/// Parse a u32 little-endian BLE value to Celsius (value ÷ 10).
pub fn raw_to_celsius_u32(bytes: &[u8]) -> Result<f32, StorzError> {
    if bytes.len() < 4 {
        return Err(StorzError::ParseError(format!(
            "Expected >= 4 bytes for u32, got {}",
            bytes.len()
        )));
    }
    let raw = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    Ok((raw as f32) / 10.0)
}

/// Build a 20-byte Venty/Veazy command buffer.
///
/// `cmd_id` goes in byte[0], `mask` in byte[1], and `payload` is a list of
/// `(offset, value)` pairs to write into the remaining bytes.
pub fn build_venty_command(cmd_id: u8, mask: u8, payload: &[(usize, u8)]) -> [u8; 20] {
    let mut buf = [0u8; 20];
    buf[0] = cmd_id;
    buf[1] = mask;
    for &(offset, value) in payload {
        if offset < 20 {
            buf[offset] = value;
        }
    }
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_raw_u32() {
        let bytes = celsius_to_raw_u32(185.0).unwrap();
        assert_eq!(u32::from_le_bytes(bytes), 1850);
    }

    #[test]
    fn test_celsius_to_raw_u16() {
        let bytes = celsius_to_raw_u16(210.0).unwrap();
        assert_eq!(u16::from_le_bytes(bytes), 2100);
    }

    #[test]
    fn test_raw_to_celsius_u16() {
        let bytes = 1850u16.to_le_bytes();
        let celsius = raw_to_celsius_u16(&bytes).unwrap();
        assert!((celsius - 185.0).abs() < 0.01);
    }

    #[test]
    fn test_temperature_range_validation() {
        assert!(celsius_to_raw_u32(30.0).is_err());
        assert!(celsius_to_raw_u32(250.0).is_err());
        assert!(celsius_to_raw_u32(40.0).is_ok());
        assert!(celsius_to_raw_u32(230.0).is_ok());
    }

    #[test]
    fn test_build_venty_command() {
        let cmd = build_venty_command(0x01, 0x02, &[(4, 0x1A), (5, 0x07)]);
        assert_eq!(cmd[0], 0x01);
        assert_eq!(cmd[1], 0x02);
        assert_eq!(cmd[4], 0x1A);
        assert_eq!(cmd[5], 0x07);
        assert_eq!(cmd[2], 0); // unset bytes remain 0
    }
}
