/// BMM350 register addresses and constant values
pub struct Register;
impl Register {
    /// Chip ID register address
    pub const CHIPID: u8 = 0x00;
    /// Error register address
    pub const ERR_REG: u8 = 0x01;
    /// Status register address
    pub const STATUS: u8 = 0x02;
    /// Magnetometer X-axis data register address (LSB)
    pub const MAG_DATA_X_LSB: u8 = 0x03;
    /// Magnetometer configuration register address
    pub const MAG_CONF: u8 = 0x20;
    /// Command register address
    pub const CMD: u8 = 0x7E;
    /// Expected chip ID for BMM350
    pub const BMM350_CHIP_ID: u8 = 0x50; // This value needs to be confirmed from the datasheet
    /// Soft reset command value
    pub const CMD_SOFT_RESET: u16 = 0xB6;
}
