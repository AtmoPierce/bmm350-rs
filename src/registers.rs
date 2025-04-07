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

    pub const OTP_CMD_REG: u8 = 0x50;

    pub const OTP_CMD_PWR_OFF_OTP: u8 = 0x80;

    pub const OTP_STATUS_REG: u8 = 0x55;

    pub const OTP_DATA_MSB_REG: u8 = 0x52;

    pub const OTP_DATA_LSB_REG: u8 = 0x53;

    pub const PMU_CMD_STATUS_0: u8 = 0x07;

    pub const PMU_CMD_STATUS_0_BR: u8 = 0x07;

    pub const PMU_CMD_STATUS_0_FGR: u8 = 0x05;

    pub const PMU_CMD_UPD_OAE: u8 = 0x02;

    pub const AVG_MASK: u8 = 0x30;

    pub const AVG_POS: u8 = 0x4;

    pub const REG_PMU_CMD: u8 = 0x06;

    pub const PMU_CMD_NM_TC: u8 = 0x09;

    pub const PMU_CMD_NM: u8 = 0x01;

    pub const PMU_CMD_SUS: u8 = 0x00;

    pub const REG_PMU_CMD_AGGR_SET: u8 = 0x04;
    
    pub const SUSPEND_TO_NORMAL_DELAY: u32 = 38_000; 

    pub const SUS_TO_FORCEDMODE_NO_AVG_DELAY: u32 = 15000;
    pub const SUS_TO_FORCEDMODE_AVG_2_DELAY: u32 = 17000;
    pub const SUS_TO_FORCEDMODE_AVG_4_DELAY: u32 = 20000;
    pub const SUS_TO_FORCEDMODE_AVG_8_DELAY: u32 = 28000;

    pub const SUS_TO_FORCEDMODE_FAST_NO_AVG_DELAY: u32 = 4000;
    pub const SUS_TO_FORCEDMODE_FAST_AVG_2_DELAY: u32 = 5000;
    pub const SUS_TO_FORCEDMODE_FAST_AVG_4_DELAY: u32 = 9000;
    pub const SUS_TO_FORCEDMODE_FAST_AVG_8_DELAY: u32 = 16000;
}
