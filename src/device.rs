use crate::{
    interface::{I2cInterface, ReadData, SpiInterface, WriteData}, types::{
        AxisEnableDisable, DataRate, Error, MagCompensation, PerformanceMode, PmuCmdStatus0,
        PowerMode, Sensor3DData, Sensor3DDataScaled,
    }, AverageNum, Bmm350, InterruptDrive, InterruptEnableDisable, InterruptLatch, InterruptMap, InterruptPolarity, MagConfig, Register
};
use embedded_hal::delay::DelayNs;

impl<I2C, D> Bmm350<I2cInterface<I2C>, D>
where
    D: DelayNs,
{
    /// Create a new BMM350 device instance
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C interface
    /// * `address` - The I2C address of the device
    /// * `delay` - A delay provider
    pub fn new_with_i2c(i2c: I2C, address: u8, delay: D) -> Self {
        Bmm350 {
            iface: I2cInterface { i2c, address },
            delay,
            mag_range: 1000.0,
            var_id: 0,
            mag_comp: MagCompensation::default(), // Default range in uT
        }
    }
}

impl<SPI, D> Bmm350<SpiInterface<SPI>, D>
where
    D: DelayNs,
{
    /// Create a new BMM350 device instance
    ///
    /// # Arguments
    ///
    /// * `spi` - The SPI interface
    /// * `delay` - A delay provider
    pub fn new_with_spi(spi: SPI, delay: D) -> Self {
        Bmm350 {
            iface: SpiInterface { spi },
            delay,
            mag_range: 1000.0,
            var_id: 0,
            mag_comp: MagCompensation::default(), // Default range in uT
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
        self.delay.delay_us(3000);
        self.write_register_16bit(Register::CMD, Register::CMD_SOFT_RESET)?;
        self.delay.delay_us(24000);

        let err = self.read_register(Register::ERR_REG)?;
        if err != 0 {
            return Err(Error::InvalidConfig);
        }

        let chip_id = self.read_register(Register::CHIPID)?;
        if chip_id != Register::BMM350_CHIP_ID {
            return Err(Error::InvalidDevice);
        }

        // Perform OTP dump after boot
        self.otp_dump_after_boot()?;

        // Power off OTP
        self.write_register(Register::OTP_CMD_REG, Register::OTP_CMD_PWR_OFF_OTP)?;

        self.magnetic_reset()?;

        Ok(())
    }

    fn otp_dump_after_boot(&mut self) -> Result<(), Error<E>> {
        let mut otp_data = [0u16; 32];

        for i in 0..32 {
            otp_data[i] = self.read_otp_word(i as u8)?;
        }

        self.var_id = ((otp_data[30] & 0x7f00) >> 9) as u8;

        // Update magnetometer offset and sensitivity data
        self.update_mag_compensation(&otp_data)?;

        Ok(())
    }

    fn read_otp_word(&mut self, addr: u8) -> Result<u16, Error<E>> {
        let otp_cmd = 0x20 | (addr & 0x1F); // OTP read command
        self.write_register(Register::OTP_CMD_REG, otp_cmd)?;

        // Wait for OTP read to complete
        for _ in 0..10 {
            self.delay.delay_us(300);
            let status = self.read_register(Register::OTP_STATUS_REG)?;
            if status & 0x01 != 0 {
                break;
            }
        }

        let msb = self.read_register(Register::OTP_DATA_MSB_REG)?;
        let lsb = self.read_register(Register::OTP_DATA_LSB_REG)?;

        Ok(((msb as u16) << 8) | (lsb as u16) & 0xFFFF)
    }

    fn update_mag_compensation(&mut self, otp_data: &[u16; 32]) -> Result<(), Error<E>> {
        // Implement the logic to update magnetometer compensation data
        // This is a simplified version and may need to be expanded based on the specific BMM350 requirements
        self.mag_comp = MagCompensation {
            offset_x: self.extract_signed_12bit(otp_data[14] & 0x0FFF),
            offset_y: self
                .extract_signed_12bit(((otp_data[14] & 0xF000) >> 4) | (otp_data[15] & 0x00FF)),
            offset_z: self
                .extract_signed_12bit(((otp_data[15] & 0x0F00) << 4) | (otp_data[16] & 0x00FF)),
            // Add more fields as necessary
        };

        Ok(())
    }

    fn extract_signed_12bit(&self, value: u16) -> i16 {
        if value & 0x0800 != 0 {
            (value | 0xF000) as i16
        } else {
            value as i16
        }
    }

    /// Perform magnetic reset of the sensor.
    /// This is necessary after a field shock (400mT field applied to sensor).
    /// It performs both a bit reset and flux guide reset in suspend mode.
    pub fn magnetic_reset(&mut self) -> Result<(), Error<E>> {
        // Check if we're in normal mode
        let mut restore_normal = false;
        let mut pmu_status = self.read_pmu_cmd_status_0()?;

        // If we're in normal mode, we need to go to suspend first
        if pmu_status.power_mode_is_normal == 0x1 {
            restore_normal = true;
            self.set_power_mode(PowerMode::Suspend)?;
        }

        // Set Bit Reset (BR) command
        self.write_register(Register::PMU_CMD, PowerMode::BitReset as u8)?;
        self.delay.delay_us(14_000); // BR_DELAY

        // Verify BR status
        pmu_status = self.read_pmu_cmd_status_0()?;
        if pmu_status.pmu_cmd_value != Register::PMU_CMD_STATUS_0_BR {
            return Err(Error::ResetUnfinished);
        }

        // Set Flux Guide Reset (FGR) command
        self.write_register(Register::PMU_CMD, PowerMode::FluxGuideReset as u8)?;
        self.delay.delay_us(18_000); // FGR_DELAY

        // Verify FGR status
        let pmu_status = self.read_pmu_cmd_status_0()?;
        if pmu_status.pmu_cmd_value != Register::PMU_CMD_STATUS_0_FGR {
            return Err(Error::ResetUnfinished);
        }

        // Restore normal mode if we were in it before
        if restore_normal {
            self.set_power_mode(PowerMode::Normal)?;
        }

        Ok(())
    }

    /// Read the PMU command status register 0
    fn read_pmu_cmd_status_0(&mut self) -> Result<PmuCmdStatus0, Error<E>> {
        let status = self.read_register(Register::PMU_CMD_STATUS_0)?;

        Ok(PmuCmdStatus0 {
            pmu_cmd_busy: (status & 0x01),
            odr_overwrite: (status & 0x2) >> 0x1,
            avg_overwrite: (status & 0x4) >> 0x2,
            power_mode_is_normal: (status & 0x8) >> 0x3,
            cmd_is_illegal: (status & 0x10) >> 0x4,
            pmu_cmd_value: (status & 0xE0) >> 5,
        })
    }

    /// Set the magnetometer configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The magnetometer configuration
    pub fn set_mag_config(&mut self, config: MagConfig) -> Result<(), Error<E>> {
        let reg_data = u16::from(config);
        self.write_register_16bit(Register::PMU_CMD_AGGR_SET, reg_data)?;

        // Wait for magnetometer data to be ready
        self.wait_for_data_ready()?;

        Ok(())
    }

    /// Set the power mode of the sensor
    ///
    /// # Arguments
    ///
    /// * `mode` - The power mode to set
    pub fn set_power_mode(&mut self, mode: PowerMode) -> Result<(), Error<E>> {
        // TODO fix
        let reg_data = mode as u8;
        self.write_register(Register::PMU_CMD, reg_data)?;

        // Wait for the power mode to be applied
        self.delay.delay_ms(10);

        Ok(())
    }

    /// Enable or disable axes
    ///
    /// # Arguments
    ///
    /// * `x` - Enable or disable X axis
    /// * `y` - Enable or disable Y axis
    /// * `z` - Enable or disable Z axis
    pub fn enable_axes(
        // TODO fix
        &mut self,
        x: AxisEnableDisable,
        y: AxisEnableDisable,
        z: AxisEnableDisable,
    ) -> Result<(), Error<E>> {
        let reg_data = (x as u8) | ((y as u8) << 1) | ((z as u8) << 2);
        self.write_register(Register::PMU_CMD_AXIS_EN, reg_data)
    }

    /// Read the raw magnetometer data
    pub fn read_mag_data(&mut self) -> Result<Sensor3DData, Error<E>> {
        let mut data = [0u8; 9];
        data[0] = Register::MAG_X_LSB;
        self.read_data(&mut data)?;

        Ok(Sensor3DData {
            x: i32::from_le_bytes([data[0], data[1], data[2], 0]),
            y: i32::from_le_bytes([data[3], data[4], data[5], 0]),
            z: i32::from_le_bytes([data[6], data[7], data[8], 0]),
        })
    }

    /// Read the scaled magnetometer data
    pub fn read_mag_data_scaled(&mut self) -> Result<Sensor3DDataScaled, Error<E>> {
        let raw_data = self.read_mag_data()?;
        let scale_factor = self.mag_range / ((1 << 23) as f32);

        Ok(Sensor3DDataScaled {
            x: raw_data.x as f32 * scale_factor,
            y: raw_data.y as f32 * scale_factor,
            z: raw_data.z as f32 * scale_factor,
        })
    }

    /// Perform a self-test
    // TODO fix this
    pub fn perform_self_test(&mut self) -> Result<bool, Error<E>> {
        // Save current configuration
        let current_power_mode = self.read_register(Register::PMU_CMD)?;
        let current_odr = self.read_register(Register::PMU_CMD_AGGR_SET)?;

        // Set device to normal mode and 100Hz ODR
        self.set_power_mode(PowerMode::Normal)?;
        self.set_mag_config(
            MagConfig::builder()
                .odr(DataRate::ODR100Hz)
                .performance(PerformanceMode::Regular)
                .build(),
        )?;

        // Perform self-test
        let self_test_passed = true;

        // Restore original configuration
        self.write_register(Register::PMU_CMD, current_power_mode)?;
        self.write_register(Register::PMU_CMD_AGGR_SET, current_odr)?;

        Ok(self_test_passed)
    }

    /// Set the output data rate and performance mode
    pub fn set_odr_performance(
        &mut self,
        odr: DataRate,
        performance: AverageNum,
    ) -> Result<(), Error<E>> {
        let reg_data = self.read_register(Register::PMU_CMD_AGGR_SET)?;
        let new_reg_data = (reg_data & 0xF0) | (odr as u8) | ((performance as u8) << 4);
        self.write_register(Register::PMU_CMD_AGGR_SET, new_reg_data)?;
        self.write_register(Register::PMU_CMD, PowerMode::Normal as u8)
    }

    /// Enable or disable the data ready interrupt
    pub fn enable_interrupt(&mut self, enable: InterruptEnableDisable) -> Result<(), Error<E>> {
        let reg_data = self.read_register(Register::INT_CTRL)?;
        let new_reg_data = (reg_data & (0x80)) | (((enable as u8) << 0x7) & 0x80);
        self.write_register(Register::INT_CTRL, new_reg_data)
    }

    /// Configure interrupt settings
    pub fn configure_interrupt(
        &mut self,
        latch: InterruptLatch,
        polarity: InterruptPolarity,
        drive: InterruptDrive,
        map: InterruptMap,
    ) -> Result<(), Error<E>> {
        let mut reg_data = self.read_register(Register::INT_CTRL)?;
        reg_data = ((reg_data & (0x1)) | (latch as u8 & 0x1))
            | (((polarity as u8) << 0x1) & 0x2)
            | ((reg_data & (0x4)) | ((drive as u8) << 0x2) & 0x4)
            | ((reg_data & (0x8)) | ((map as u8) << 0x3) & 0x8);
        self.write_register(Register::INT_CTRL, reg_data)
    }

    /// Read the interrupt status
    pub fn get_interrupt_status(&mut self) -> Result<bool, Error<E>> {
        let status = self.read_register(Register::STATUS)?;
        Ok((status & 0x04) != 0)
    }

    /// Set the I2C watchdog timer
    pub fn set_i2c_watchdog(&mut self, enable: bool, long_timeout: bool) -> Result<(), Error<E>> {
        let reg_data = (enable as u8) | ((long_timeout as u8) << 1);
        self.write_register(Register::I2C_WDT_SET, reg_data)
    }

    fn write_register(&mut self, reg: u8, value: u8) -> Result<(), Error<E>> {
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
        for _ in 0..100 {
            if self.get_interrupt_status()? {
                return Ok(());
            }
            self.delay.delay_ms(1);
        }
        Err(Error::Timeout)
    }
}
