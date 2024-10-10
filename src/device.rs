use crate::{
    interface::{I2cInterface, ReadData, SpiInterface, WriteData},
    types::{MagnetometerRange, Sensor3DData, Sensor3DDataScaled, SensorType},
    Bmm350, Error, MagConfig, Register,
};
use embedded_hal::delay::DelayNs;

impl<I2C, D> Bmm350<I2cInterface<I2C>, D>
where
    D: DelayNs,
{
    /// Create a new BMM350 device instance
    pub fn new_with_i2c(i2c: I2C, address: u8, delay: D) -> Self {
        Bmm350 {
            iface: I2cInterface { i2c, address },
            delay,
            mag_range: MagnetometerRange::default(),
        }
    }
}

impl<SPI, D> Bmm350<SpiInterface<SPI>, D>
where
    D: DelayNs,
{
    /// Create a new BMM350 device instance
    pub fn new_with_spi(spi: SPI, delay: D) -> Self {
        Bmm350 {
            iface: SpiInterface { spi },
            delay,
            mag_range: MagnetometerRange::default(),
        }
    }
}

impl<DI, D, E> Bmm350<DI, D>
where
    DI: ReadData<Error = Error<E>> + WriteData<Error = Error<E>>,
    D: DelayNs,
{
    /// Initialize the device
    pub fn init(&mut self) -> Result<(), Error<E>> {
        self.write_register_8bit(Register::CMD, Register::CMD_SOFT_RESET)?;
        self.delay.delay_ms(10);

        let chip_id = self.read_register(Register::CHIPID)?;
        if chip_id != Register::BMM350_CHIP_ID {
            return Err(Error::InvalidDevice);
        }

        Ok(())
    }

    /// Set the magnetometer configuration
    pub fn set_mag_config(&mut self, config: MagConfig) -> Result<(), Error<E>> {
        let reg_data = self.config_to_reg_data(config);
        self.write_register_16bit(Register::MAG_CONF, reg_data)?;
        self.mag_range = config.range;

        // Wait for magnetometer data to be ready
        self.wait_for_data_ready()?;

        Ok(())
    }

    fn config_to_reg_data<T>(&self, config: T) -> u16
    where
        T: Into<u16> + Copy,
    {
        let config: u16 = config.into();
        config
    }

    /// Read the raw magnetometer data
    pub fn read_mag_data(&mut self) -> Result<Sensor3DData, Error<E>> {
        let mut data = [0u8; 7];
        data[0] = Register::MAG_DATA_X_LSB;
        let sensor_data = self.read_data(&mut data)?;

        Ok(Sensor3DData {
            x: i16::from_le_bytes([sensor_data[0], sensor_data[1]]),
            y: i16::from_le_bytes([sensor_data[2], sensor_data[3]]),
            z: i16::from_le_bytes([sensor_data[4], sensor_data[5]]),
        })
    }

    /// Read the magnetometer data and return the scaled value in µT
    pub fn read_mag_data_scaled(&mut self) -> Result<Sensor3DDataScaled, Error<E>> {
        let raw_data = self.read_mag_data()?;
        Ok(raw_data.to_ut(self.mag_range.to_ut()))
    }

    fn write_register_8bit(&mut self, reg: u8, value: u8) -> Result<(), Error<E>> {
        self.iface.write_data(&[reg, value])
    }

    fn write_register_16bit(&mut self, reg: u8, value: u16) -> Result<(), Error<E>> {
        let bytes = value.to_le_bytes();
        self.iface.write_data(&[reg, bytes[0], bytes[1]])
    }

    fn read_register(&mut self, reg: u8) -> Result<u8, Error<E>> {
        self.iface.read_register(reg)
    }

    fn read_data<'a>(&mut self, data: &'a mut [u8]) -> Result<&'a [u8], Error<E>> {
        self.iface.read_data(data)
    }

    fn wait_for_data_ready(&mut self) -> Result<(), Error<E>> {
        const MAX_RETRIES: u8 = 100;
        let mut retries = 0;

        while !self.is_data_ready()? {
            if retries >= MAX_RETRIES {
                return Err(Error::Timeout);
            }
            self.delay.delay_ms(1);
            retries += 1;
        }

        Ok(())
    }

    fn is_data_ready(&mut self) -> Result<bool, Error<E>> {
        let status = self.read_register(Register::STATUS)?;
        Ok((status & 0b0000_0001) != 0) // Check bit 0 (drdy_mag)
    }
}

// ... (keep the test module as is, updating it for magnetometer data if needed)
