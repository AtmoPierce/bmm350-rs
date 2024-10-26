/// BMM350 register addresses and constant values
pub struct Register;
impl Register {
    /// Chip ID register address
    pub const CHIPID: u8 = 0x00;
    /// Error register address
    pub const ERR_REG: u8 = 0x02;
    /// Status register address
    pub const STATUS: u8 = 0x03;
    /// Magnetometer X-axis data register address
    pub const MAG_X_LSB: u8 = 0x31;
    /// Magnetometer configuration register address
    pub const PMU_CMD_AGGR_SET: u8 = 0x04;
    /// Axis enable register address
    pub const PMU_CMD_AXIS_EN: u8 = 0x05;
    /// Power mode command register address
    pub const PMU_CMD: u8 = 0x06;
    /// Command register address
    pub const CMD: u8 = 0x7E;
    /// Expected chip ID for BMM350
    pub const BMM350_CHIP_ID: u8 = 0x33;
    /// Soft reset command value
    pub const CMD_SOFT_RESET: u16 = 0xB6;

    pub const TMR_SELFTEST_USER: u8 = 0x60;

    pub const INT_CTRL: u8 = 0x2E;

    pub const I2C_WDT_SET: u8 = 0x0A;
}
