use crate::{
    bmm3_defs as defs,
    bmm3_enums::Bmm3Error,
    bmm3_structs::{Bmm350Dev, Bmm350PmuCmdStatus0},
    bmm3_types::Bmm3Result,
};

impl Bmm350Dev {
    pub fn null_ptr_check(&self) -> Bmm3Result<()> {
        if self.read.is_none() || self.write.is_none() || self.delay_us.is_none() {
            Err(Bmm3Error::NullPtr)
        } else {
            Ok(())
        }
    }

    pub fn bmm350_set_regs(&mut self, reg_addr: u8, reg_data: &[u8], len: u16) -> Bmm3Result<()> {
        self.null_ptr_check()?;

        if reg_data.is_empty() {
            return Err(Bmm3Error::NullPtr);
        }
        match self.write.unwrap()(reg_addr, reg_data, len as u32, self.intf_ptr) {
            Ok(()) => Ok(()),
            _ => Err(Bmm3Error::ComFail),
        }
    }

    pub fn bmm350_get_regs(
        &mut self,
        reg_addr: u8,
        reg_data: &mut [u8],
        len: u16,
    ) -> Bmm3Result<()> {
        self.null_ptr_check()?;

        let temp_len: u16 = len + defs::BMM350_DUMMY_BYTES as u16;

        let mut temp_buf = [0u8, defs::BMM350_READ_BUFFER_LENGTH];

        match self.read.unwrap()(reg_addr, &mut temp_buf, temp_len as u32, self.intf_ptr) {
            Ok(()) => {
                for i in 0..len as usize {
                    reg_data[i] = temp_buf[i + defs::BMM350_DUMMY_BYTES as usize];
                }
                Ok(())
            }
            _ => Err(Bmm3Error::ComFail),
        }
    }

    pub fn bmm350_soft_reset(&mut self) -> Bmm3Result<()> {
        self.null_ptr_check()?;

        let mut otp_cmd = [defs::BMM350_OTP_CMD_PWR_OFF_OTP];

        let mut reg_data = [defs::BMM350_CMD_SOFTRESET];

        self.bmm350_set_regs(defs::BMM350_REG_CMD, &reg_data, 1)?;
        self.delay_us.unwrap()(defs::BMM350_SOFT_RESET_DELAY, self.intf_ptr);
        self.bmm350_set_regs(defs::BMM350_REG_OTP_CMD_REG, &reg_data, 1)?;

        Ok(())
    }

    // TODO use another Implement for the CmdStatus0 struct in order to make an update function that I could pass in rather than doing it all here
    pub fn bmm350_get_pmu_cmd_status_0(&mut self, bmm350_pmu_cmd_status0: &mut Bmm350PmuCmdStatus0) {
        let mut reg_data = [0u8];
        self.bmm350_get_regs(defs::BMM350_REG_PMU_CMD_STATUS_0, &mut reg_data, 1);
    }
}

impl Bmm350PmuCmdStatus0 {
    pub fn update(&mut self, reg_data: u8) {
        self.cmd_drdy = (reg_data & defs::BMM350_PMU_CMD_STATUS_0_CMD_DRDY) != 0;
        self.cmd_err = (reg_data & defs::BMM350_PMU_CMD_STATUS_0_CMD_ERR) != 0;
        self.mdr_err = (reg_data & defs::BMM350_PMU_CMD_STATUS_0_MDR_ERR) != 0;
        self.mag_data_rdy = (reg_data & defs::BMM350_PMU_CMD_STATUS_0_MAG_DATA_RDY) != 0;
        self.mag_data_ovrflow = (reg_data & defs::BMM350_PMU_CMD_STATUS_0_MAG_DATA_OVRFLOW) != 0;
    }
}