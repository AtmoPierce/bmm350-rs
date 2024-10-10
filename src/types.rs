use core::fmt::Debug;

/// Possible errors that can occur when interacting with the BMM350
#[derive(Debug)]
pub enum Error<E> {
    /// Communication error
    Comm(E),
    /// Invalid device (wrong chip ID)
    InvalidDevice,
    /// Invalid configuration
    InvalidConfig,
    /// Timeout error
    Timeout,
}

/// Magnetometer power modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MagnetometerPowerMode {
    /// Sleep mode
    Sleep = 0x00,
    /// Low power mode
    LowPower = 0x01,
    /// Normal power mode
    Normal = 0x02,
    /// High performance mode
    HighPerf = 0x03,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MagnetometerRange {
    /// ±1300 µT
    uT1300 = 0,
    /// ±2600 µT
    uT2600 = 1,
    /// ±3900 µT
    uT3900 = 2,
    /// ±5200 µT
    uT5200 = 3,
}

impl MagnetometerRange {
    pub fn to_ut(self) -> f32 {
        match self {
            MagnetometerRange::uT1300 => 1300.0,
            MagnetometerRange::uT2600 => 2600.0,
            MagnetometerRange::uT3900 => 3900.0,
            MagnetometerRange::uT5200 => 5200.0,
        }
    }
}

impl Default for MagnetometerRange {
    fn default() -> Self {
        MagnetometerRange::uT1300
    }
}

// ... (keep the rest of the file as is, including Sensor3DData and Sensor3DDataScaled)

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SensorType {
    Magnetometer,
}
