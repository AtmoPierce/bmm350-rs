use crate::bmm3_types::{Bmm350MrawOverride, I2cResult, Bmm350ReadPtr};
use crate::bmm3_defs::BMM350_OTP_DATA_LENGTH;
// use crate::bmm3_defs::BMM350_OTP_DATA_LENGTH as length;

/// BMM350 device struct
#[repr(C)]
pub struct Bmm350Dev {
    pub chip_id: u8,
    pub intf_ptr: *mut (),
    pub intf_rslt: i8,
    pub read: Option<Bmm350ReadPtr>,
    pub write: Option<fn(reg_addr: u8, reg_data: &[u8], length: u32, intf_ptr: *mut ()) -> I2cResult<()>>,
    pub delay_us: Option<fn(period: u32, intf_ptr: *mut ())>,
    pub axis_en: u8,
    pub var_id: u8,
    pub otp_data: [u16; BMM350_OTP_DATA_LENGTH as usize],
    pub mraw_override: Option<Bmm350MrawOverride>,
}

/// raw mag data
pub struct Bmm350RawData {
    pub raw_xdata: i32,
    pub raw_ydata: i32,
    pub raw_zdata: i32,
    pub raw_temp: i32,
}

/// bmm350 compensated magnetometer data and temperature data
pub struct Bmm350MagTempData {
    /// Compensated mag X data
    pub x: f32,
    /// Compensated mag Y data
    pub y: f32,
    /// Compensated mag Z data
    pub z: f32,
    /// Temperature
    pub temperature: f32,
}

/// bmm350 magnetometer dut offset coefficient structure
pub struct Bmm350DutOffsetCoef {
    /// Temperature offset
    pub t_offs: f32,
    /// Offset x-axis
    pub offset_x: f32,
    /// Offset y-axis
    pub offset_y: f32,
    /// Offset z-axis
    pub offset_z: f32,
}

/// bmm350 magnetometer dut sensitivity coefficient structure
pub struct Bmm350DutSensitCoef {
    /// Temperature sensitivity
    pub t_sens: f32,
    /// Sensitivity x-axis
    pub sens_x: f32,
    /// Sensitivity y-axis
    pub sens_y: f32,
    /// Sensitivity z-axis
    pub sens_z: f32,
}

/// bmm350 magnetometer dut tco structure
pub struct Bmm350DutTco {
    pub tco_x: f32,
    pub tco_y: f32,
    pub tco_z: f32,
}

/// bmm350 magnetometer dut tcs structure
pub struct Bmm350DutTcs {
    pub tcs_x: f32,
    pub tcs_y: f32,
    pub tcs_z: f32,
}

/// bmm350 magnetometer cross axis compensation structure
pub struct Bmm350CrossAxis {
    pub cross_x_y: f32,
    pub cross_y_x: f32,
    pub cross_z_x: f32,
    pub cross_z_y: f32,
}

/// bmm350 magnetometer compensate structure
pub struct Bmm350MagCompensate {
    /// Structure to store dut offset coefficient
    pub dut_offset_coef: Bmm350DutOffsetCoef,
    /// Structure to store dut sensitivity coefficient
    pub dut_sensit_coef: Bmm350DutSensitCoef,
    /// Structure to store dut tco
    pub dut_tco: Bmm350DutTco,
    /// Structure to store dut tcs
    pub dut_tcs: Bmm350DutTcs,
    /// Initialize T0_reading parameter
    pub dut_t0: f32,
    /// Structure to define cross axis compensation
    pub cross_axis: Bmm350CrossAxis,
}

/// bmm350 self-test structure
pub struct Bmm350SelfTest {
    /// Variable to store self-test data on x-axis
    pub out_ust_x: f32,
    /// Variable to store self-test data on y-axis
    pub out_ust_y: f32,
}

/// bmm350 PMU command status 0 structure
pub struct Bmm350PmuCmdStatus0 {
    /// The previous PMU CMD is still in processing
    pub pmu_cmd_busy: u8,
    /// The previous PMU_CMD_AGGR_SET.odr has been overwritten
    pub odr_ovwr: u8,
    /// The previous PMU_CMD_AGGR_SET.avg has been overwritten
    pub avr_ovwr: u8,
    /// The chip is in normal power mode
    pub pwr_mode_is_normal: u8,
    /// CMD value is not allowed
    pub cmd_is_illegal: u8,
    /// Stores the latest PMU_CMD code processed
    pub pmu_cmd_value: u8,
}