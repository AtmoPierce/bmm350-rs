#[derive(Debug, PartialEq, Eq)]
pub enum I2cError {
    NullPtr,
    ComFail,
    NackReceived,
    InitializationError,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Bmm3Error {
    NullPtr,
    ComFail,
    DevNotFound,
    InvalidCfg,
    BadPadDrive,
    ResetUnfinished,
    InvalidInput,
    SelfTestInvalidAxis,
    OtpBoot,
    OtpPageRd,
    OtpPagePrg,
    OtpSign,
    OtpInvCMD,
    OtpUndefined,
    AllAxisDisable,
    PmuCMDValue
}