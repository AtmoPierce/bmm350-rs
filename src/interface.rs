use crate::Error;
use embedded_hal::{i2c, spi::SpiDevice};

/// I2C communication interface for BMM350
#[derive(Debug)]
pub struct I2cInterface<I2C> {
    pub(crate) i2c: I2C,
    pub(crate) address: u8,
}

/// SPI communication interface for BMM350
#[derive(Debug)]
pub struct SpiInterface<SPI> {
    pub(crate) spi: SPI,
}

/// Trait for writing data to the BMM350
pub trait WriteData {
    type Error;
    /// Write data to the device
    fn write_data(&mut self, payload: &[u8]) -> Result<(), Self::Error>;
}

impl<I2C, E> WriteData for I2cInterface<I2C>
where
    I2C: i2c::I2c<Error = E>,
{
    type Error = Error<E>;
    fn write_data(&mut self, payload: &[u8]) -> Result<(), Self::Error> {
        self.i2c.write(self.address, payload).map_err(Error::Comm)
    }
}

impl<SPI, E> WriteData for SpiInterface<SPI>
where
    SPI: SpiDevice<Error = E>,
{
    type Error = Error<E>;
    fn write_data(&mut self, payload: &[u8]) -> Result<(), Self::Error> {
        self.spi.write(payload).map_err(Error::Comm)
    }
}

pub trait ReadData {
    type Error;
    /// Read data from the device
    fn read_data(&mut self, register: u8, data: &mut [u8]) -> Result<(), Self::Error>;
}

impl<I2C, E> ReadData for I2cInterface<I2C>
where
    I2C: i2c::I2c<Error = E>,
{
    type Error = Error<E>;
    fn read_data(&mut self, register: u8, data: &mut [u8]) -> Result<(), Error<E>> {
        self.i2c
            .write_read(self.address, &[register], data)
            .map_err(Error::Comm)
    }
}

impl<SPI, E> ReadData for SpiInterface<SPI>
where
    SPI: SpiDevice<Error = E>,
{
    type Error = Error<E>;
    fn read_data(&mut self, register: u8, data: &mut [u8]) -> Result<(), Error<E>> {
        let mut buffer = [register | 0x80; 32];
        buffer[1..data.len() + 1].copy_from_slice(data);
        self.spi
            .transfer_in_place(&mut buffer[..data.len() + 1])
            .map_err(Error::Comm)?;
        data.copy_from_slice(&buffer[1..data.len() + 1]);
        Ok(())
    }
}
