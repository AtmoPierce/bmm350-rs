/* Chip id of BMM350 */
pub const BMM350_CHIP_ID                              :u8 = 0x33;

/* Variant ID of BMM350 */
pub const BMM350_MIN_VAR                              :u8 = 0x10;

/************************* Sensor interface success code **************************/
pub const BMM350_INTF_RET_SUCCESS                     :i8 = 0;

/************************* API success code **************************/
pub const BMM350_OK                                   :i8 = 0;

/* API error codes */
pub const BMM350_E_NULL_PTR                           :i8 = -1;
pub const BMM350_E_COM_FAIL                           :i8 = -2;
pub const BMM350_E_DEV_NOT_FOUND                      :i8 = -3;
pub const BMM350_E_INVALID_CONFIG                     :i8 = -4;
pub const BMM350_E_BAD_PAD_DRIVE                      :i8 = -5;
pub const BMM350_E_RESET_UNFINISHED                   :i8 = -6;
pub const BMM350_E_INVALID_INPUT                      :i8 = -7;
pub const BMM350_E_SELF_TEST_INVALID_AXIS             :i8 = -8;
pub const BMM350_E_OTP_BOOT                           :i8 = -9;
pub const BMM350_E_OTP_PAGE_RD                        :i8 = -10;
pub const BMM350_E_OTP_PAGE_PRG                       :i8 = -11;
pub const BMM350_E_OTP_SIGN                           :i8 = -12;
pub const BMM350_E_OTP_INV_CMD                        :i8 = -13;
pub const BMM350_E_OTP_UNDEFINED                      :i8 = -14;
pub const BMM350_E_ALL_AXIS_DISABLED                  :i8 = -15;
pub const BMM350_E_PMU_CMD_VALUE                      :i8 = -16;

pub const BMM350_NO_ERROR                             :u8 = 0;

/************************* Sensor delay time settings in microseconds **************************/
pub const BMM350_SOFT_RESET_DELAY                     :u32 = 24000;
pub const BMM350_MAGNETIC_RESET_DELAY                 :u32 = 40000;
pub const BMM350_START_UP_TIME_FROM_POR               :u32 = 3000;

pub const BMM350_GOTO_SUSPEND_DELAY                   :u32 = 6000;
pub const BMM350_SUSPEND_TO_NORMAL_DELAY              :u32 = 38000;

pub const BMM350_SUS_TO_FORCEDMODE_NO_AVG_DELAY       :u32 = 15000;
pub const BMM350_SUS_TO_FORCEDMODE_AVG_2_DELAY        :u32 = 17000;
pub const BMM350_SUS_TO_FORCEDMODE_AVG_4_DELAY        :u32 = 20000;
pub const BMM350_SUS_TO_FORCEDMODE_AVG_8_DELAY        :u32 = 28000;

pub const BMM350_SUS_TO_FORCEDMODE_FAST_NO_AVG_DELAY  :u32 = 4000;
pub const BMM350_SUS_TO_FORCEDMODE_FAST_AVG_2_DELAY   :u32 = 5000;
pub const BMM350_SUS_TO_FORCEDMODE_FAST_AVG_4_DELAY   :u32 = 9000;
pub const BMM350_SUS_TO_FORCEDMODE_FAST_AVG_8_DELAY   :u32 = 16000;

pub const BMM350_UPD_OAE_DELAY                        :u16 = 1000;

pub const BMM350_BR_DELAY                             :u16 = 14000;
pub const BMM350_FGR_DELAY                            :u16 = 18000;

/************************ Length macros ************************/
pub const BMM350_OTP_DATA_LENGTH                      :u8 = 32;
pub const BMM350_READ_BUFFER_LENGTH                   :u8 = 127;
pub const BMM350_MAG_TEMP_DATA_LEN                    :u8 = 12;

/************************ Averaging macros **********************/
pub const BMM350_AVG_NO_AVG                           :u8 = 0x0;
pub const BMM350_AVG_2                                :u8 = 0x1;
pub const BMM350_AVG_4                                :u8 = 0x2;
pub const BMM350_AVG_8                                :u8 = 0x3;

/******************************* ODR **************************/
pub const BMM350_ODR_400HZ                            :u8 = 0x2;
pub const BMM350_ODR_200HZ                            :u8 = 0x3;
pub const BMM350_ODR_100HZ                            :u8 = 0x4;
pub const BMM350_ODR_50HZ                             :u8 = 0x5;
pub const BMM350_ODR_25HZ                             :u8 = 0x6;
pub const BMM350_ODR_12_5HZ                           :u8 = 0x7;
pub const BMM350_ODR_6_25HZ                           :u8 = 0x8;
pub const BMM350_ODR_3_125HZ                          :u8 = 0x9;
pub const BMM350_ODR_1_5625HZ                         :u8 = 0xA;

/********************* Power modes *************************/
pub const BMM350_PMU_CMD_SUS                          :u8 = 0x00;
pub const BMM350_PMU_CMD_NM                           :u8 = 0x01;
pub const BMM350_PMU_CMD_UPD_OAE                      :u8 = 0x02;
pub const BMM350_PMU_CMD_FM                           :u8 = 0x03;
pub const BMM350_PMU_CMD_FM_FAST                      :u8 = 0x04;
pub const BMM350_PMU_CMD_FGR                          :u8 = 0x05;
pub const BMM350_PMU_CMD_FGR_FAST                     :u8 = 0x06;
pub const BMM350_PMU_CMD_BR                           :u8 = 0x07;
pub const BMM350_PMU_CMD_BR_FAST                      :u8 = 0x08;
pub const BMM350_PMU_CMD_NM_TC                        :u8 = 0x09;

pub const BMM350_PMU_STATUS_0                         :u8 = 0x0;

pub const BMM350_DISABLE                              :u8 = 0x0;
pub const BMM350_ENABLE                               :u8 = 0x1;

pub const BMM350_CMD_NOP                              :u8 = 0x0;
pub const BMM350_CMD_SOFTRESET                        :u8 = 0xB6;

pub const BMM350_TARGET_PAGE_PAGE0                    :u8 = 0x0;
pub const BMM350_TARGET_PAGE_PAGE1                    :u8 = 0x1;

pub const BMM350_INT_MODE_LATCHED                     :u8 = 0x1;
pub const BMM350_INT_MODE_PULSED                      :u8 = 0x0;

pub const BMM350_INT_POL_ACTIVE_HIGH                  :u8 = 0x1;
pub const BMM350_INT_POL_ACTIVE_LOW                   :u8 = 0x0;

pub const BMM350_INT_OD_PUSHPULL                      :u8 = 0x1;
pub const BMM350_INT_OD_OPENDRAIN                     :u8 = 0x0;

pub const BMM350_INT_OUTPUT_EN_OFF                    :u8 = 0x0;
pub const BMM350_INT_OUTPUT_EN_ON                     :u8 = 0x1;

pub const BMM350_INT_DRDY_EN                          :u8 = 0x1;
pub const BMM350_INT_DRDY_DIS                         :u8 = 0x0;

pub const BMM350_MR_MR1K8                             :u8 = 0x0;
pub const BMM350_MR_MR2K1                             :u8 = 0x1;
pub const BMM350_MR_MR1K5                             :u8 = 0x2;
pub const BMM350_MR_MR0K6                             :u8 = 0x3;

pub const BMM350_SEL_DTB1X_PAD_PAD_INT                :u8 = 0x0;
pub const BMM350_SEL_DTB1X_PAD_PAD_BYP                :u8 = 0x1;

pub const BMM350_TMR_TST_HIZ_VTMR_VTMR_ON             :u8 = 0x0;
pub const BMM350_TMR_TST_HIZ_VTMR_VTMR_HIZ            :u8 = 0x1;

pub const BMM350_LSB_MASK                             :u16 = 0x00FF;
pub const BMM350_MSB_MASK                             :u16 = 0xFF00;

/********************** Pad drive strength ************************/
pub const BMM350_PAD_DRIVE_WEAKEST                    :u8 = 0;
pub const BMM350_PAD_DRIVE_STRONGEST                  :u8 = 7;

/********************** I2C Register Addresses ************************/

/* Register to set I2C address to LOW */
pub const BMM350_I2C_ADSEL_SET_LOW                    :u8 = 0x14;

/* Register to set I2C address to HIGH */
pub const BMM350_I2C_ADSEL_SET_HIGH                   :u8 = 0x15;

pub const BMM350_DUMMY_BYTES                          :u8 = 2;

/********************** Register Addresses ************************/

pub const BMM350_REG_CHIP_ID                          :u8 = 0x00;
pub const BMM350_REG_REV_ID                           :u8 = 0x01;
pub const BMM350_REG_ERR_REG                          :u8 = 0x02;
pub const BMM350_REG_PAD_CTRL                         :u8 = 0x03;
pub const BMM350_REG_PMU_CMD_AGGR_SET                 :u8 = 0x04;
pub const BMM350_REG_PMU_CMD_AXIS_EN                  :u8 = 0x05;
pub const BMM350_REG_PMU_CMD                          :u8 = 0x06;
pub const BMM350_REG_PMU_CMD_STATUS_0                 :u8 = 0x07;
pub const BMM350_REG_PMU_CMD_STATUS_1                 :u8 = 0x08;
pub const BMM350_REG_I3C_ERR                          :u8 = 0x09;
pub const BMM350_REG_I2C_WDT_SET                      :u8 = 0x0A;
pub const BMM350_REG_TRSDCR_REV_ID                    :u8 = 0x0D;
pub const BMM350_REG_TC_SYNC_TU                       :u8 = 0x21;
pub const BMM350_REG_TC_SYNC_ODR                      :u8 = 0x22;
pub const BMM350_REG_TC_SYNC_TPH_1                    :u8 = 0x23;
pub const BMM350_REG_TC_SYNC_TPH_2                    :u8 = 0x24;
pub const BMM350_REG_TC_SYNC_DT                       :u8 = 0x25;
pub const BMM350_REG_TC_SYNC_ST_0                     :u8 = 0x26;
pub const BMM350_REG_TC_SYNC_ST_1                     :u8 = 0x27;
pub const BMM350_REG_TC_SYNC_ST_2                     :u8 = 0x28;
pub const BMM350_REG_TC_SYNC_STATUS                   :u8 = 0x29;
pub const BMM350_REG_INT_CTRL                         :u8 = 0x2E;
pub const BMM350_REG_INT_CTRL_IBI                     :u8 = 0x2F;
pub const BMM350_REG_INT_STATUS                       :u8 = 0x30;
pub const BMM350_REG_MAG_X_XLSB                       :u8 = 0x31;
pub const BMM350_REG_MAG_X_LSB                        :u8 = 0x32;
pub const BMM350_REG_MAG_X_MSB                        :u8 = 0x33;
pub const BMM350_REG_MAG_Y_XLSB                       :u8 = 0x34;
pub const BMM350_REG_MAG_Y_LSB                        :u8 = 0x35;
pub const BMM350_REG_MAG_Y_MSB                        :u8 = 0x36;
pub const BMM350_REG_MAG_Z_XLSB                       :u8 = 0x37;
pub const BMM350_REG_MAG_Z_LSB                        :u8 = 0x38;
pub const BMM350_REG_MAG_Z_MSB                        :u8 = 0x39;
pub const BMM350_REG_TEMP_XLSB                        :u8 = 0x3A;
pub const BMM350_REG_TEMP_LSB                         :u8 = 0x3B;
pub const BMM350_REG_TEMP_MSB                         :u8 = 0x3C;
pub const BMM350_REG_SENSORTIME_XLSB                  :u8 = 0x3D;
pub const BMM350_REG_SENSORTIME_LSB                   :u8 = 0x3E;
pub const BMM350_REG_SENSORTIME_MSB                   :u8 = 0x3F;
pub const BMM350_REG_OTP_CMD_REG                      :u8 = 0x50;
pub const BMM350_REG_OTP_DATA_MSB_REG                 :u8 = 0x52;
pub const BMM350_REG_OTP_DATA_LSB_REG                 :u8 = 0x53;
pub const BMM350_REG_OTP_STATUS_REG                   :u8 = 0x55;
pub const BMM350_REG_TMR_SELFTEST_USER                :u8 = 0x60;
pub const BMM350_REG_CTRL_USER                        :u8 = 0x61;
pub const BMM350_REG_CMD                              :u8 = 0x7E;

/*********************** Macros for OVWR ***************************/
pub const BMM350_REG_OVWR_VALUE_ANA_0                 :u8 = 0x3A;
pub const BMM350_REG_OVWR_EN_ANA_0                    :u8 = 0x3B;

/*********************** Macros for bit masking ***************************/

pub const BMM350_CHIP_ID_OTP_MSK                      :u8 = 0xf;
pub const BMM350_CHIP_ID_OTP_POS                      :u8 = 0x0;
pub const BMM350_CHIP_ID_FIXED_MSK                    :u8 = 0xf0;
pub const BMM350_CHIP_ID_FIXED_POS                    :u8 = 0x4;
pub const BMM350_REV_ID_MAJOR_MSK                     :u8 = 0xf0;
pub const BMM350_REV_ID_MAJOR_POS                     :u8 = 0x4;
pub const BMM350_REV_ID_MINOR_MSK                     :u8 = 0xf;
pub const BMM350_REV_ID_MINOR_POS                     :u8 = 0x0;
pub const BMM350_PMU_CMD_ERROR_MSK                    :u8 = 0x1;
pub const BMM350_PMU_CMD_ERROR_POS                    :u8 = 0x0;
pub const BMM350_BOOT_UP_ERROR_MSK                    :u8 = 0x2;
pub const BMM350_BOOT_UP_ERROR_POS                    :u8 = 0x1;
pub const BMM350_DRV_MSK                              :u8 = 0x7;
pub const BMM350_DRV_POS                              :u8 = 0x0;
pub const BMM350_AVG_MSK                              :u8 = 0x30;
pub const BMM350_AVG_POS                              :u8 = 0x4;
pub const BMM350_ODR_MSK                              :u8 = 0xf;
pub const BMM350_ODR_POS                              :u8 = 0x0;
pub const BMM350_PMU_CMD_MSK                          :u8 = 0xf;
pub const BMM350_PMU_CMD_POS                          :u8 = 0x0;
pub const BMM350_EN_X_MSK                             :u8 = 0x01;
pub const BMM350_EN_X_POS                             :u8 = 0x0;
pub const BMM350_EN_Y_MSK                             :u8 = 0x02;
pub const BMM350_EN_Y_POS                             :u8 = 0x1;
pub const BMM350_EN_Z_MSK                             :u8 = 0x04;
pub const BMM350_EN_Z_POS                             :u8 = 0x2;
pub const BMM350_EN_XYZ_MSK                           :u8 = 0x7;
pub const BMM350_EN_XYZ_POS                           :u8 = 0x0;
pub const BMM350_PMU_CMD_BUSY_MSK                     :u8 = 0x1;
pub const BMM350_PMU_CMD_BUSY_POS                     :u8 = 0x0;
pub const BMM350_ODR_OVWR_MSK                         :u8 = 0x2;
pub const BMM350_ODR_OVWR_POS                         :u8 = 0x1;
pub const BMM350_AVG_OVWR_MSK                         :u8 = 0x4;
pub const BMM350_AVG_OVWR_POS                         :u8 = 0x2;
pub const BMM350_PWR_MODE_IS_NORMAL_MSK               :u8 = 0x8;
pub const BMM350_PWR_MODE_IS_NORMAL_POS               :u8 = 0x3;
pub const BMM350_CMD_IS_ILLEGAL_MSK                   :u8 = 0x10;
pub const BMM350_CMD_IS_ILLEGAL_POS                   :u8 = 0x4;
pub const BMM350_PMU_CMD_VALUE_MSK                    :u8 = 0xE0;
pub const BMM350_PMU_CMD_VALUE_POS                    :u8 = 0x5;
pub const BMM350_PMU_ODR_S_MSK                        :u8 = 0xf;
pub const BMM350_PMU_ODR_S_POS                        :u8 = 0x0;
pub const BMM350_PMU_AVG_S_MSK                        :u8 = 0x30;
pub const BMM350_PMU_AVG_S_POS                        :u8 = 0x4;
pub const BMM350_I3C_ERROR_0_MSK                      :u8 = 0x1;
pub const BMM350_I3C_ERROR_0_POS                      :u8 = 0x0;
pub const BMM350_I3C_ERROR_3_MSK                      :u8 = 0x8;
pub const BMM350_I3C_ERROR_3_POS                      :u8 = 0x3;
pub const BMM350_I2C_WDT_EN_MSK                       :u8 = 0x1;
pub const BMM350_I2C_WDT_EN_POS                       :u8 = 0x0;
pub const BMM350_I2C_WDT_SEL_MSK                      :u8 = 0x2;
pub const BMM350_I2C_WDT_SEL_POS                      :u8 = 0x1;
pub const BMM350_TRSDCR_REV_ID_OTP_MSK                :u8 = 0x3;
pub const BMM350_TRSDCR_REV_ID_OTP_POS                :u8 = 0x0;
pub const BMM350_TRSDCR_REV_ID_FIXED_MSK              :u8 = 0xfc;
pub const BMM350_TRSDCR_REV_ID_FIXED_POS              :u8 = 0x2;
pub const BMM350_PAGING_EN_MSK                        :u8 = 0x80;
pub const BMM350_PAGING_EN_POS                        :u8 = 0x7;
pub const BMM350_DRDY_DATA_REG_MSK                    :u8 = 0x4;
pub const BMM350_DRDY_DATA_REG_POS                    :u8 = 0x2;
pub const BMM350_INT_MODE_MSK                         :u8 = 0x1;
pub const BMM350_INT_MODE_POS                         :u8 = 0x0;
pub const BMM350_INT_POL_MSK                          :u8 = 0x2;
pub const BMM350_INT_POL_POS                          :u8 = 0x1;
pub const BMM350_INT_OD_MSK                           :u8 = 0x4;
pub const BMM350_INT_OD_POS                           :u8 = 0x2;
pub const BMM350_INT_OUTPUT_EN_MSK                    :u8 = 0x8;
pub const BMM350_INT_OUTPUT_EN_POS                    :u8 = 0x3;
pub const BMM350_DRDY_DATA_REG_EN_MSK                 :u8 = 0x80;
pub const BMM350_DRDY_DATA_REG_EN_POS                 :u8 = 0x7;
pub const BMM350_DRDY_INT_MAP_TO_IBI_MSK              :u8 = 0x1;
pub const BMM350_DRDY_INT_MAP_TO_IBI_POS              :u8 = 0x0;
pub const BMM350_CLEAR_DRDY_INT_STATUS_UPON_IBI_MSK   :u8 = 0x10;
pub const BMM350_CLEAR_DRDY_INT_STATUS_UPON_IBI_POS   :u8 = 0x4;
pub const BMM350_TC_SYNC_TU_MSK                       :u8 = 0xff;
pub const BMM350_TC_SYNC_ODR_MSK                      :u8 = 0xff;
pub const BMM350_TC_SYNC_TPH_1_MSK                    :u8 = 0xff;
pub const BMM350_TC_SYNC_TPH_2_MSK                    :u8 = 0xff;
pub const BMM350_TC_SYNC_DT_MSK                       :u8 = 0xff;
pub const BMM350_TC_SYNC_ST_0_MSK                     :u8 = 0xff;
pub const BMM350_TC_SYNC_ST_1_MSK                     :u8 = 0xff;
pub const BMM350_TC_SYNC_ST_2_MSK                     :u8 = 0xff;
pub const BMM350_CFG_FORCE_SOSC_EN_MSK                :u8 = 0x4;
pub const BMM350_CFG_FORCE_SOSC_EN_POS                :u8 = 0x2;
pub const BMM350_ST_IGEN_EN_MSK                       :u8 = 0x1;
pub const BMM350_ST_IGEN_EN_POS                       :u8 = 0x0;
pub const BMM350_ST_N_MSK                             :u8 = 0x2;
pub const BMM350_ST_N_POS                             :u8 = 0x1;
pub const BMM350_ST_P_MSK                             :u8 = 0x4;
pub const BMM350_ST_P_POS                             :u8 = 0x2;
pub const BMM350_IST_EN_X_MSK                         :u8 = 0x8;
pub const BMM350_IST_EN_X_POS                         :u8 = 0x3;
pub const BMM350_IST_EN_Y_MSK                         :u8 = 0x10;
pub const BMM350_IST_EN_Y_POS                         :u8 = 0x4;
pub const BMM350_CFG_SENS_TIM_AON_MSK                 :u8 = 0x1;
pub const BMM350_CFG_SENS_TIM_AON_POS                 :u8 = 0x0;
pub const BMM350_DATA_X_7_0_MSK                       :u8 = 0xff;
pub const BMM350_DATA_X_7_0_POS                       :u8 = 0x0;
pub const BMM350_DATA_X_15_8_MSK                      :u8 = 0xff;
pub const BMM350_DATA_X_15_8_POS                      :u8 = 0x0;
pub const BMM350_DATA_X_23_16_MSK                     :u8 = 0xff;
pub const BMM350_DATA_X_23_16_POS                     :u8 = 0x0;
pub const BMM350_DATA_Y_7_0_MSK                       :u8 = 0xff;
pub const BMM350_DATA_Y_7_0_POS                       :u8 = 0x0;
pub const BMM350_DATA_Y_15_8_MSK                      :u8 = 0xff;
pub const BMM350_DATA_Y_15_8_POS                      :u8 = 0x0;
pub const BMM350_DATA_Y_23_16_MSK                     :u8 = 0xff;
pub const BMM350_DATA_Y_23_16_POS                     :u8 = 0x0;
pub const BMM350_DATA_Z_7_0_MSK                       :u8 = 0xff;
pub const BMM350_DATA_Z_7_0_POS                       :u8 = 0x0;
pub const BMM350_DATA_Z_15_8_MSK                      :u8 = 0xff;
pub const BMM350_DATA_Z_15_8_POS                      :u8 = 0x0;
pub const BMM350_DATA_Z_23_16_MSK                     :u8 = 0xff;
pub const BMM350_DATA_Z_23_16_POS                     :u8 = 0x0;
pub const BMM350_DATA_T_7_0_MSK                       :u8 = 0xff;
pub const BMM350_DATA_T_7_0_POS                       :u8 = 0x0;
pub const BMM350_DATA_T_15_8_MSK                      :u8 = 0xff;
pub const BMM350_DATA_T_15_8_POS                      :u8 = 0x0;
pub const BMM350_DATA_T_23_16_MSK                     :u8 = 0xff;
pub const BMM350_DATA_T_23_16_POS                     :u8 = 0x0;
pub const BMM350_DATA_ST_7_0_MSK                      :u8 = 0xff;
pub const BMM350_DATA_ST_7_0_POS                      :u8 = 0x0;
pub const BMM350_DATA_ST_15_8_MSK                     :u8 = 0xff;
pub const BMM350_DATA_ST_15_8_POS                     :u8 = 0x0;
pub const BMM350_DATA_ST_23_16_MSK                    :u8 = 0xff;
pub const BMM350_DATA_ST_23_16_POS                    :u8 = 0x0;
pub const BMM350_SIGN_INVERT_T_MSK                    :u8 = 0x10;
pub const BMM350_SIGN_INVERT_T_POS                    :u8 = 0x4;
pub const BMM350_SIGN_INVERT_X_MSK                    :u8 = 0x20;
pub const BMM350_SIGN_INVERT_X_POS                    :u8 = 0x5;
pub const BMM350_SIGN_INVERT_Y_MSK                    :u8 = 0x40;
pub const BMM350_SIGN_INVERT_Y_POS                    :u8 = 0x6;
pub const BMM350_SIGN_INVERT_Z_MSK                    :u8 = 0x80;
pub const BMM350_SIGN_INVERT_Z_POS                    :u8 = 0x7;
pub const BMM350_DIS_BR_NM_MSK                        :u8 = 0x1;
pub const BMM350_DIS_BR_NM_POS                        :u8 = 0x0;
pub const BMM350_DIS_FGR_NM_MSK                       :u8 = 0x2;
pub const BMM350_DIS_FGR_NM_POS                       :u8 = 0x1;
pub const BMM350_DIS_CRST_AT_ALL_MSK                  :u8 = 0x4;
pub const BMM350_DIS_CRST_AT_ALL_POS                  :u8 = 0x2;
pub const BMM350_DIS_BR_FM_MSK                        :u8 = 0x8;
pub const BMM350_DIS_BR_FM_POS                        :u8 = 0x3;
pub const BMM350_FRC_EN_BUFF_MSK                      :u8 = 0x1;
pub const BMM350_FRC_EN_BUFF_POS                      :u8 = 0x0;
pub const BMM350_FRC_INA_EN1_MSK                      :u8 = 0x2;
pub const BMM350_FRC_INA_EN1_POS                      :u8 = 0x1;
pub const BMM350_FRC_INA_EN2_MSK                      :u8 = 0x4;
pub const BMM350_FRC_INA_EN2_POS                      :u8 = 0x2;
pub const BMM350_FRC_ADC_EN_MSK                       :u8 = 0x8;
pub const BMM350_FRC_ADC_EN_POS                       :u8 = 0x3;
pub const BMM350_FRC_INA_RST_MSK                      :u8 = 0x10;
pub const BMM350_FRC_INA_RST_POS                      :u8 = 0x4;
pub const BMM350_FRC_ADC_RST_MSK                      :u8 = 0x20;
pub const BMM350_FRC_ADC_RST_POS                      :u8 = 0x5;
pub const BMM350_FRC_INA_XSEL_MSK                     :u8 = 0x1;
pub const BMM350_FRC_INA_XSEL_POS                     :u8 = 0x0;
pub const BMM350_FRC_INA_YSEL_MSK                     :u8 = 0x2;
pub const BMM350_FRC_INA_YSEL_POS                     :u8 = 0x1;
pub const BMM350_FRC_INA_ZSEL_MSK                     :u8 = 0x4;
pub const BMM350_FRC_INA_ZSEL_POS                     :u8 = 0x2;
pub const BMM350_FRC_ADC_TEMP_EN_MSK                  :u8 = 0x8;
pub const BMM350_FRC_ADC_TEMP_EN_POS                  :u8 = 0x3;
pub const BMM350_FRC_TSENS_EN_MSK                     :u8 = 0x10;
pub const BMM350_FRC_TSENS_EN_POS                     :u8 = 0x4;
pub const BMM350_DSENS_FM_MSK                         :u8 = 0x20;
pub const BMM350_DSENS_FM_POS                         :u8 = 0x5;
pub const BMM350_DSENS_SEL_MSK                        :u8 = 0x40;
pub const BMM350_DSENS_SEL_POS                        :u8 = 0x6;
pub const BMM350_DSENS_SHORT_MSK                      :u8 = 0x80;
pub const BMM350_DSENS_SHORT_POS                      :u8 = 0x7;
pub const BMM350_ERR_MISS_BR_DONE_MSK                 :u8 = 0x1;
pub const BMM350_ERR_MISS_BR_DONE_POS                 :u8 = 0x0;
pub const BMM350_ERR_MISS_FGR_DONE_MSK                :u8 = 0x2;
pub const BMM350_ERR_MISS_FGR_DONE_POS                :u8 = 0x1;
pub const BMM350_TST_CHAIN_LN_MODE_MSK                :u8 = 0x1;
pub const BMM350_TST_CHAIN_LN_MODE_POS                :u8 = 0x0;
pub const BMM350_TST_CHAIN_LP_MODE_MSK                :u8 = 0x2;
pub const BMM350_TST_CHAIN_LP_MODE_POS                :u8 = 0x1;
pub const BMM350_EN_OVWR_TMR_IF_MSK                   :u8 = 0x1;
pub const BMM350_EN_OVWR_TMR_IF_POS                   :u8 = 0x0;
pub const BMM350_TMR_CKTRIGB_MSK                      :u8 = 0x2;
pub const BMM350_TMR_CKTRIGB_POS                      :u8 = 0x1;
pub const BMM350_TMR_DO_BR_MSK                        :u8 = 0x4;
pub const BMM350_TMR_DO_BR_POS                        :u8 = 0x2;
pub const BMM350_TMR_DO_FGR_MSK                       :u8 = 0x18;
pub const BMM350_TMR_DO_FGR_POS                       :u8 = 0x3;
pub const BMM350_TMR_EN_OSC_MSK                       :u8 = 0x80;
pub const BMM350_TMR_EN_OSC_POS                       :u8 = 0x7;
pub const BMM350_VCM_TRIM_X_MSK                       :u8 = 0x1f;
pub const BMM350_VCM_TRIM_X_POS                       :u8 = 0x0;
pub const BMM350_VCM_TRIM_Y_MSK                       :u8 = 0x1f;
pub const BMM350_VCM_TRIM_Y_POS                       :u8 = 0x0;
pub const BMM350_VCM_TRIM_Z_MSK                       :u8 = 0x1f;
pub const BMM350_VCM_TRIM_Z_POS                       :u8 = 0x0;
pub const BMM350_VCM_TRIM_DSENS_MSK                   :u8 = 0x1f;
pub const BMM350_VCM_TRIM_DSENS_POS                   :u8 = 0x0;
pub const BMM350_TWLB_MSK                             :u8 = 0x30;
pub const BMM350_TWLB_POS                             :u8 = 0x4;
pub const BMM350_PRG_PLS_TIM_MSK                      :u8 = 0x30;
pub const BMM350_PRG_PLS_TIM_POS                      :u8 = 0x4;
pub const BMM350_OTP_OVWR_EN_MSK                      :u8 = 0x1;
pub const BMM350_OTP_OVWR_EN_POS                      :u8 = 0x0;
pub const BMM350_OTP_MEM_CLK_MSK                      :u8 = 0x2;
pub const BMM350_OTP_MEM_CLK_POS                      :u8 = 0x1;
pub const BMM350_OTP_MEM_CS_MSK                       :u8 = 0x4;
pub const BMM350_OTP_MEM_CS_POS                       :u8 = 0x2;
pub const BMM350_OTP_MEM_PGM_MSK                      :u8 = 0x8;
pub const BMM350_OTP_MEM_PGM_POS                      :u8 = 0x3;
pub const BMM350_OTP_MEM_RE_MSK                       :u8 = 0x10;
pub const BMM350_OTP_MEM_RE_POS                       :u8 = 0x4;
pub const BMM350_SAMPLE_RDATA_PLS_MSK                 :u8 = 0x80;
pub const BMM350_SAMPLE_RDATA_PLS_POS                 :u8 = 0x7;
pub const BMM350_CFG_FW_MSK                           :u8 = 0x1;
pub const BMM350_CFG_FW_POS                           :u8 = 0x0;
pub const BMM350_EN_BR_X_MSK                          :u8 = 0x2;
pub const BMM350_EN_BR_X_POS                          :u8 = 0x1;
pub const BMM350_EN_BR_Y_MSK                          :u8 = 0x4;
pub const BMM350_EN_BR_Y_POS                          :u8 = 0x2;
pub const BMM350_EN_BR_Z_MSK                          :u8 = 0x8;
pub const BMM350_EN_BR_Z_POS                          :u8 = 0x3;
pub const BMM350_CFG_PAUSE_TIME_MSK                   :u8 = 0x30;
pub const BMM350_CFG_PAUSE_TIME_POS                   :u8 = 0x4;
pub const BMM350_CFG_FGR_PLS_DUR_MSK                  :u8 = 0xf;
pub const BMM350_CFG_FGR_PLS_DUR_POS                  :u8 = 0x0;
pub const BMM350_CFG_BR_Z_ORDER_MSK                   :u8 = 0x10;
pub const BMM350_CFG_BR_Z_ORDER_POS                   :u8 = 0x4;
pub const BMM350_CFG_BR_XY_CHOP_MSK                   :u8 = 0x20;
pub const BMM350_CFG_BR_XY_CHOP_POS                   :u8 = 0x5;
pub const BMM350_CFG_BR_PLS_DUR_MSK                   :u8 = 0xc0;
pub const BMM350_CFG_BR_PLS_DUR_POS                   :u8 = 0x6;
pub const BMM350_ENABLE_BR_FGR_TEST_MSK               :u8 = 0x1;
pub const BMM350_ENABLE_BR_FGR_TEST_POS               :u8 = 0x0;
pub const BMM350_SEL_AXIS_MSK                         :u8 = 0xe;
pub const BMM350_SEL_AXIS_POS                         :u8 = 0x1;
pub const BMM350_TMR_CFG_TEST_CLK_EN_MSK              :u8 = 0x10;
pub const BMM350_TMR_CFG_TEST_CLK_EN_POS              :u8 = 0x4;
pub const BMM350_TEST_VAL_BITS_7DOWNTO0_MSK           :u8 = 0xff;
pub const BMM350_TEST_VAL_BITS_7DOWNTO0_POS           :u8 = 0x0;
pub const BMM350_TEST_VAL_BITS_8_MSK                  :u8 = 0x1;
pub const BMM350_TEST_VAL_BITS_8_POS                  :u8 = 0x0;
pub const BMM350_TEST_P_SAMPLE_MSK                    :u8 = 0x2;
pub const BMM350_TEST_P_SAMPLE_POS                    :u8 = 0x1;
pub const BMM350_TEST_N_SAMPLE_MSK                    :u8 = 0x4;
pub const BMM350_TEST_N_SAMPLE_POS                    :u8 = 0x2;
pub const BMM350_TEST_APPLY_TO_REM_MSK                :u8 = 0x10;
pub const BMM350_TEST_APPLY_TO_REM_POS                :u8 = 0x4;
pub const BMM350_UFO_TRM_OSC_RANGE_MSK                :u8 = 0xf;
pub const BMM350_UFO_TRM_OSC_RANGE_POS                :u8 = 0x0;
pub const BMM350_ISO_CHIP_ID_MSK                      :u8 = 0x78;
pub const BMM350_ISO_CHIP_ID_POS                      :u8 = 0x3;
pub const BMM350_ISO_I2C_DEV_ID_MSK                   :u8 = 0x80;
pub const BMM350_ISO_I2C_DEV_ID_POS                   :u8 = 0x7;
pub const BMM350_I3C_FREQ_BITS_1DOWNTO0_MSK           :u8 = 0xc;
pub const BMM350_I3C_FREQ_BITS_1DOWNTO0_POS           :u8 = 0x2;
pub const BMM350_I3C_IBI_MDB_SEL_MSK                  :u8 = 0x10;
pub const BMM350_I3C_IBI_MDB_SEL_POS                  :u8 = 0x4;
pub const BMM350_TC_ASYNC_EN_MSK                      :u8 = 0x20;
pub const BMM350_TC_ASYNC_EN_POS                      :u8 = 0x5;
pub const BMM350_TC_SYNC_EN_MSK                       :u8 = 0x40;
pub const BMM350_TC_SYNC_EN_POS                       :u8 = 0x6;
pub const BMM350_I3C_SCL_GATING_EN_MSK                :u8 = 0x80;
pub const BMM350_I3C_SCL_GATING_EN_POS                :u8 = 0x7;
pub const BMM350_I3C_INACCURACY_BITS_6DOWNTO0_MSK     :u8 = 0x7f;
pub const BMM350_I3C_INACCURACY_BITS_6DOWNTO0_POS     :u8 = 0x0;
pub const BMM350_EST_EN_X_MSK                         :u8 = 0x1;
pub const BMM350_EST_EN_X_POS                         :u8 = 0x0;
pub const BMM350_EST_EN_Y_MSK                         :u8 = 0x2;
pub const BMM350_EST_EN_Y_POS                         :u8 = 0x1;
pub const BMM350_CRST_DIS_MSK                         :u8 = 0x4;
pub const BMM350_CRST_DIS_POS                         :u8 = 0x2;
pub const BMM350_BR_TFALL_MSK                         :u8 = 0x7;
pub const BMM350_BR_TFALL_POS                         :u8 = 0x0;
pub const BMM350_BR_TRISE_MSK                         :u8 = 0x70;
pub const BMM350_BR_TRISE_POS                         :u8 = 0x4;
pub const BMM350_TMR_SOFT_START_DIS_MSK               :u8 = 0x80;
pub const BMM350_TMR_SOFT_START_DIS_POS               :u8 = 0x7;
pub const BMM350_FOSC_LOW_RANGE_MSK                   :u8 = 0x80;
pub const BMM350_FOSC_LOW_RANGE_POS                   :u8 = 0x7;
pub const BMM350_VCRST_TRIM_FG_MSK                    :u8 = 0x3f;
pub const BMM350_VCRST_TRIM_FG_POS                    :u8 = 0x0;
pub const BMM350_VCRST_TRIM_BR_MSK                    :u8 = 0x3f;
pub const BMM350_VCRST_TRIM_BR_POS                    :u8 = 0x0;
pub const BMM350_BG_TRIM_VRP_MSK                      :u8 = 0xc0;
pub const BMM350_BG_TRIM_VRP_POS                      :u8 = 0x6;
pub const BMM350_BG_TRIM_TC_MSK                       :u8 = 0xf;
pub const BMM350_BG_TRIM_TC_POS                       :u8 = 0x0;
pub const BMM350_BG_TRIM_VRA_MSK                      :u8 = 0xf0;
pub const BMM350_BG_TRIM_VRA_POS                      :u8 = 0x4;
pub const BMM350_BG_TRIM_VRD_MSK                      :u8 = 0xf;
pub const BMM350_BG_TRIM_VRD_POS                      :u8 = 0x0;
pub const BMM350_OVWR_REF_IB_EN_MSK                   :u8 = 0x10;
pub const BMM350_OVWR_REF_IB_EN_POS                   :u8 = 0x4;
pub const BMM350_OVWR_VDDA_EN_MSK                     :u8 = 0x20;
pub const BMM350_OVWR_VDDA_EN_POS                     :u8 = 0x5;
pub const BMM350_OVWR_VDDP_EN_MSK                     :u8 = 0x40;
pub const BMM350_OVWR_VDDP_EN_POS                     :u8 = 0x6;
pub const BMM350_OVWR_VDDS_EN_MSK                     :u8 = 0x80;
pub const BMM350_OVWR_VDDS_EN_POS                     :u8 = 0x7;
pub const BMM350_REF_IB_EN_MSK                        :u8 = 0x10;
pub const BMM350_REF_IB_EN_POS                        :u8 = 0x4;
pub const BMM350_VDDA_EN_MSK                          :u8 = 0x20;
pub const BMM350_VDDA_EN_POS                          :u8 = 0x5;
pub const BMM350_VDDP_EN_MSK                          :u8 = 0x40;
pub const BMM350_VDDP_EN_POS                          :u8 = 0x6;
pub const BMM350_VDDS_EN_MSK                          :u8 = 0x80;
pub const BMM350_VDDS_EN_POS                          :u8 = 0x7;
pub const BMM350_OVWR_OTP_PROG_VDD_SW_EN_MSK          :u8 = 0x8;
pub const BMM350_OVWR_OTP_PROG_VDD_SW_EN_POS          :u8 = 0x3;
pub const BMM350_OVWR_EN_MFE_BG_FILT_BYPASS_MSK       :u8 = 0x10;
pub const BMM350_OVWR_EN_MFE_BG_FILT_BYPASS_POS       :u8 = 0x4;
pub const BMM350_OTP_PROG_VDD_SW_EN_MSK               :u8 = 0x8;
pub const BMM350_OTP_PROG_VDD_SW_EN_POS               :u8 = 0x3;
pub const BMM350_CP_COMP_CRST_EN_TM_MSK               :u8 = 0x10;
pub const BMM350_CP_COMP_CRST_EN_TM_POS               :u8 = 0x4;
pub const BMM350_CP_COMP_VDD_EN_TM_MSK                :u8 = 0x20;
pub const BMM350_CP_COMP_VDD_EN_TM_POS                :u8 = 0x5;
pub const BMM350_CP_INTREFS_EN_TM_MSK                 :u8 = 0x40;
pub const BMM350_CP_INTREFS_EN_TM_POS                 :u8 = 0x6;
pub const BMM350_ADC_LOCAL_CHOP_EN_MSK                :u8 = 0x20;
pub const BMM350_ADC_LOCAL_CHOP_EN_POS                :u8 = 0x5;
pub const BMM350_INA_MODE_MSK                         :u8 = 0x40;
pub const BMM350_INA_MODE_POS                         :u8 = 0x6;
pub const BMM350_VDDD_EXT_EN_MSK                      :u8 = 0x20;
pub const BMM350_VDDD_EXT_EN_POS                      :u8 = 0x5;
pub const BMM350_VDDP_EXT_EN_MSK                      :u8 = 0x80;
pub const BMM350_VDDP_EXT_EN_POS                      :u8 = 0x7;
pub const BMM350_ADC_DSENS_EN_MSK                     :u8 = 0x10;
pub const BMM350_ADC_DSENS_EN_POS                     :u8 = 0x4;
pub const BMM350_DSENS_EN_MSK                         :u8 = 0x20;
pub const BMM350_DSENS_EN_POS                         :u8 = 0x5;
pub const BMM350_OTP_TM_CLVWR_EN_MSK                  :u8 = 0x40;
pub const BMM350_OTP_TM_CLVWR_EN_POS                  :u8 = 0x6;
pub const BMM350_OTP_VDDP_DIS_MSK                     :u8 = 0x80;
pub const BMM350_OTP_VDDP_DIS_POS                     :u8 = 0x7;
pub const BMM350_FORCE_HIGH_VREF_IREF_OK_MSK          :u8 = 0x10;
pub const BMM350_FORCE_HIGH_VREF_IREF_OK_POS          :u8 = 0x4;
pub const BMM350_FORCE_HIGH_FOSC_OK_MSK               :u8 = 0x20;
pub const BMM350_FORCE_HIGH_FOSC_OK_POS               :u8 = 0x5;
pub const BMM350_FORCE_HIGH_MFE_BG_RDY_MSK            :u8 = 0x40;
pub const BMM350_FORCE_HIGH_MFE_BG_RDY_POS            :u8 = 0x6;
pub const BMM350_FORCE_HIGH_MFE_VTMR_RDY_MSK          :u8 = 0x80;
pub const BMM350_FORCE_HIGH_MFE_VTMR_RDY_POS          :u8 = 0x7;
pub const BMM350_ERR_END_OF_RECHARGE_MSK              :u8 = 0x1;
pub const BMM350_ERR_END_OF_RECHARGE_POS              :u8 = 0x0;
pub const BMM350_ERR_END_OF_DISCHARGE_MSK             :u8 = 0x2;
pub const BMM350_ERR_END_OF_DISCHARGE_POS             :u8 = 0x1;
pub const BMM350_CP_TMX_DIGTP_SEL_MSK                 :u8 = 0x7;
pub const BMM350_CP_TMX_DIGTP_SEL_POS                 :u8 = 0x0;
pub const BMM350_CP_CPOSC_EN_TM_MSK                   :u8 = 0x80;
pub const BMM350_CP_CPOSC_EN_TM_POS                   :u8 = 0x7;
pub const BMM350_TST_ATM1_CFG_MSK                     :u8 = 0x3f;
pub const BMM350_TST_ATM1_CFG_POS                     :u8 = 0x0;
pub const BMM350_TST_TB1_EN_MSK                       :u8 = 0x80;
pub const BMM350_TST_TB1_EN_POS                       :u8 = 0x7;
pub const BMM350_TST_ATM2_CFG_MSK                     :u8 = 0x1f;
pub const BMM350_TST_ATM2_CFG_POS                     :u8 = 0x0;
pub const BMM350_TST_TB2_EN_MSK                       :u8 = 0x80;
pub const BMM350_TST_TB2_EN_POS                       :u8 = 0x7;
pub const BMM350_REG_DTB1X_SEL_MSK                    :u8 = 0x7f;
pub const BMM350_REG_DTB1X_SEL_POS                    :u8 = 0x0;
pub const BMM350_SEL_DTB1X_PAD_MSK                    :u8 = 0x80;
pub const BMM350_SEL_DTB1X_PAD_POS                    :u8 = 0x7;
pub const BMM350_REG_DTB2X_SEL_MSK                    :u8 = 0x7f;
pub const BMM350_REG_DTB2X_SEL_POS                    :u8 = 0x0;
pub const BMM350_TMR_TST_CFG_MSK                      :u8 = 0x7f;
pub const BMM350_TMR_TST_CFG_POS                      :u8 = 0x0;
pub const BMM350_TMR_TST_HIZ_VTMR_MSK                 :u8 = 0x80;
pub const BMM350_TMR_TST_HIZ_VTMR_POS                 :u8 = 0x7;

/****************************** OTP MACROS ***************************/
pub const BMM350_OTP_CMD_DIR_READ                     :u8 = 0x20;
pub const BMM350_OTP_CMD_DIR_PRGM_1B                  :u8 = 0x40;
pub const BMM350_OTP_CMD_DIR_PRGM                     :u8 = 0x60;
pub const BMM350_OTP_CMD_PWR_OFF_OTP                  :u8 = 0x80;
pub const BMM350_OTP_CMD_EXT_READ                     :u8 = 0xA0;
pub const BMM350_OTP_CMD_EXT_PRGM                     :u8 = 0xE0;
pub const BMM350_OTP_CMD_MSK                          :u8 = 0xE0;
pub const BMM350_OTP_WORD_ADDR_MSK                    :u8 = 0x1F;

pub const BMM350_OTP_STATUS_ERROR_MSK                 :u8 = 0xE0;
// pub const BMM350_OTP_STATUS_ERROR(val;                (val & BMM350_OTP_STATUS_ERROR_MSK;
pub const BMM350_OTP_STATUS_NO_ERROR                  :u8 = 0x00;
pub const BMM350_OTP_STATUS_BOOT_ERR                  :u8 = 0x20;
pub const BMM350_OTP_STATUS_PAGE_RD_ERR               :u8 = 0x40;
pub const BMM350_OTP_STATUS_PAGE_PRG_ERR              :u8 = 0x60;
pub const BMM350_OTP_STATUS_SIGN_ERR                  :u8 = 0x80;
pub const BMM350_OTP_STATUS_INV_CMD_ERR               :u8 = 0xA0;
pub const BMM350_OTP_STATUS_CMD_DONE                  :u8 = 0x01;

/****************************** OTP indices ***************************/
pub const BMM350_TEMP_OFF_SENS                        :u8 = 0x0D;

pub const BMM350_MAG_OFFSET_X                         :u8 = 0x0E;
pub const BMM350_MAG_OFFSET_Y                         :u8 = 0x0F;
pub const BMM350_MAG_OFFSET_Z                         :u8 = 0x10;

pub const BMM350_MAG_SENS_X                           :u8 = 0x10;
pub const BMM350_MAG_SENS_Y                           :u8 = 0x11;
pub const BMM350_MAG_SENS_Z                           :u8 = 0x11;

pub const BMM350_MAG_TCO_X                            :u8 = 0x12;
pub const BMM350_MAG_TCO_Y                            :u8 = 0x13;
pub const BMM350_MAG_TCO_Z                            :u8 = 0x14;

pub const BMM350_MAG_TCS_X                            :u8 = 0x12;
pub const BMM350_MAG_TCS_Y                            :u8 = 0x13;
pub const BMM350_MAG_TCS_Z                            :u8 = 0x14;

pub const BMM350_MAG_DUT_T_0                          :u8 = 0x18;

pub const BMM350_CROSS_X_Y                            :u8 = 0x15;
pub const BMM350_CROSS_Y_X                            :u8 = 0x15;
pub const BMM350_CROSS_Z_X                            :u8 = 0x16;
pub const BMM350_CROSS_Z_Y                            :u8 = 0x16;

// pub const BMM350_SENS_CORR_Y                          (0.01f;
// pub const BMM350_TCS_CORR_Z                           (0.0001f;

/**************************** Signed bit macros **********************/
pub const BMM350_SIGNED_8_BIT                         :u8 = 8;
pub const BMM350_SIGNED_12_BIT                        :u8 = 12;
pub const BMM350_SIGNED_16_BIT                        :u8 = 16;
pub const BMM350_SIGNED_21_BIT                        :u8 = 21;
pub const BMM350_SIGNED_24_BIT                        :u8 = 24;

/**************************** Self-test macros **********************/
pub const BMM350_SELF_TEST_DISABLE                    :u8 = 0x00;
pub const BMM350_SELF_TEST_POS_X                      :u8 = 0x0D;
pub const BMM350_SELF_TEST_NEG_X                      :u8 = 0x0B;
pub const BMM350_SELF_TEST_POS_Y                      :u8 = 0x15;
pub const BMM350_SELF_TEST_NEG_Y                      :u8 = 0x13;

pub const BMM350_X_FM_XP_UST_MAX_LIMIT                :i16 = 150;
pub const BMM350_X_FM_XP_UST_MIN_LIMIT                :i16 = 50;

pub const BMM350_X_FM_XN_UST_MAX_LIMIT                :i16 = -50;
pub const BMM350_X_FM_XN_UST_MIN_LIMIT                :i16 = -150;

pub const BMM350_Y_FM_YP_UST_MAX_LIMIT                :i16 = 150;
pub const BMM350_Y_FM_YP_UST_MIN_LIMIT                :i16 = 50;

pub const BMM350_Y_FM_YN_UST_MAX_LIMIT                :i16 = -50;
pub const BMM350_Y_FM_YN_UST_MIN_LIMIT                :i16 = -150;

/**************************** PMU command status 0 macros **********************/
pub const BMM350_PMU_CMD_STATUS_0_SUS                 :u8 = 0x00;
pub const BMM350_PMU_CMD_STATUS_0_NM                  :u8 = 0x01;
pub const BMM350_PMU_CMD_STATUS_0_UPD_OAE             :u8 = 0x02;
pub const BMM350_PMU_CMD_STATUS_0_FM                  :u8 = 0x03;
pub const BMM350_PMU_CMD_STATUS_0_FM_FAST             :u8 = 0x04;
pub const BMM350_PMU_CMD_STATUS_0_FGR                 :u8 = 0x05;
pub const BMM350_PMU_CMD_STATUS_0_FGR_FAST            :u8 = 0x06;
pub const BMM350_PMU_CMD_STATUS_0_BR                  :u8 = 0x07;
pub const BMM350_PMU_CMD_STATUS_0_BR_FAST             :u8 = 0x07;