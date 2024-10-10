#![no_std]

/// BMM350 driver for Rust
///
/// This module provides a high-level interface for interacting with the Bosch BMM350 geomagnetic sensor.
/// It supports both I2C and SPI interfaces and allows for configuration of the magnetometer settings.
pub mod device;
mod interface;
mod registers;
pub use registers::Register;
mod types;
pub use types::{
    Error, MagnetometerPowerMode, MagnetometerRange, OutputDataRate, Sensor3DData,
    Sensor3DDataScaled,
};
mod sensor_data;
pub use sensor_data::*;

/// Main struct representing the BMM350 device
pub struct Bmm350<DI, D> {
    /// Communication interface (I2C or SPI)
    iface: DI,
    /// Delay provider
    delay: D,
    /// Current magnetometer range
    mag_range: MagnetometerRange,
}

/// Configuration for the magnetometer
#[derive(Debug, Clone, Copy)]
pub struct MagConfig {
    /// Output data rate
    pub odr: OutputDataRate,
    /// Measurement range
    pub range: MagnetometerRange,
    /// Power mode
    pub mode: MagnetometerPowerMode,
}

impl MagConfig {
    /// Create a new MagConfigBuilder
    pub fn builder() -> MagConfigBuilder {
        MagConfigBuilder::default()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MagConfigBuilder {
    odr: Option<OutputDataRate>,
    range: Option<MagnetometerRange>,
    mode: Option<MagnetometerPowerMode>,
}

/// Builder for MagConfig
impl Default for MagConfigBuilder {
    fn default() -> Self {
        Self {
            odr: None,
            range: None,
            mode: None,
        }
    }
}

impl MagConfigBuilder {
    /// Set the output data rate
    pub fn odr(mut self, odr: OutputDataRate) -> Self {
        self.odr = Some(odr);
        self
    }

    /// Set the measurement range
    pub fn range(mut self, range: MagnetometerRange) -> Self {
        self.range = Some(range);
        self
    }

    /// Set the power mode
    pub fn mode(mut self, mode: MagnetometerPowerMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Build the MagConfig
    pub fn build(self) -> MagConfig {
        MagConfig {
            odr: self.odr.unwrap_or(OutputDataRate::Odr20hz),
            range: self.range.unwrap_or(MagnetometerRange::uT1300),
            mode: self.mode.unwrap_or(MagnetometerPowerMode::Normal),
        }
    }
}

impl From<MagConfig> for u16 {
    /// Convert MagConfig to a 16-bit register value
    fn from(config: MagConfig) -> Self {
        (config.odr as u16 & 0x0F)
            | ((config.range as u16 & 0x07) << 4)
            | ((config.mode as u16 & 0x03) << 7)
    }
}
