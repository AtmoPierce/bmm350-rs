use crate::bmm3_enums::{I2cError, Bmm3Error};
use crate::bmm3_structs::Bmm350Dev;

pub type I2cResult<T> = Result<T, I2cError>;
pub type Bmm3Result<T> = core::result::Result<T, Bmm3Error>;

pub type Bmm350MrawOverride = fn(&mut Bmm350Dev) -> i8;

pub type Bmm350ReadPtr = fn(reg_addr: u8, reg_data: &mut [u8], length: u32, intf_ptr: *mut ()) -> I2cResult<()>;