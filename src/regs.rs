#![allow(dead_code)]

pub const BNO055_DEFAULT_ADDR: u8 = 0x29;
pub const BNO055_ALTERNATE_ADDR: u8 = 0x28;
pub const BNO055_ID: u8 = 0xA0;

pub const BNO055_PAGE_ID: u8 = 0x07;

pub const BNO055_CHIP_ID: u8 = 0x00;
pub const BNO055_ACC_ID: u8 = 0x01;
pub const BNO055_MAG_ID: u8 = 0x02;
pub const BNO055_GYR_ID: u8 = 0x03;
pub const BNO055_SW_REV_ID_LSB: u8 = 0x04;
pub const BNO055_SW_REV_ID_MSB: u8 = 0x05;
pub const BNO055_BL_REV_ID: u8 = 0x06;

pub const BNO055_ACC_DATA_X_LSB: u8 = 0x08;
pub const BNO055_ACC_DATA_X_MSB: u8 = 0x09;
pub const BNO055_ACC_DATA_Y_LSB: u8 = 0x0A;
pub const BNO055_ACC_DATA_Y_MSB: u8 = 0x0B;
pub const BNO055_ACC_DATA_Z_LSB: u8 = 0x0C;
pub const BNO055_ACC_DATA_Z_MSB: u8 = 0x0D;

pub const BNO055_MAG_DATA_X_LSB: u8 = 0x0E;
pub const BNO055_MAG_DATA_X_MSB: u8 = 0x0F;
pub const BNO055_MAG_DATA_Y_LSB: u8 = 0x10;
pub const BNO055_MAG_DATA_Y_MSB: u8 = 0x11;
pub const BNO055_MAG_DATA_Z_LSB: u8 = 0x12;
pub const BNO055_MAG_DATA_Z_MSB: u8 = 0x13;

pub const BNO055_GYR_DATA_X_LSB: u8 = 0x14;
pub const BNO055_GYR_DATA_X_MSB: u8 = 0x15;
pub const BNO055_GYR_DATA_Y_LSB: u8 = 0x16;
pub const BNO055_GYR_DATA_Y_MSB: u8 = 0x17;
pub const BNO055_GYR_DATA_Z_LSB: u8 = 0x18;
pub const BNO055_GYR_DATA_Z_MSB: u8 = 0x19;

pub const BNO055_EUL_HEADING_LSB: u8 = 0x1A;
pub const BNO055_EUL_HEADING_MSB: u8 = 0x1B;
pub const BNO055_EUL_ROLL_LSB: u8 = 0x1C;
pub const BNO055_EUL_ROLL_MSB: u8 = 0x1D;
pub const BNO055_EUL_PITCH_LSB: u8 = 0x1E;
pub const BNO055_EUL_PITCH_MSB: u8 = 0x1F;

/// Quaternion data
pub const BNO055_QUA_DATA_W_LSB: u8 = 0x20;
pub const BNO055_QUA_DATA_W_MSB: u8 = 0x21;
pub const BNO055_QUA_DATA_X_LSB: u8 = 0x22;
pub const BNO055_QUA_DATA_X_MSB: u8 = 0x23;
pub const BNO055_QUA_DATA_Y_LSB: u8 = 0x24;
pub const BNO055_QUA_DATA_Y_MSB: u8 = 0x25;
pub const BNO055_QUA_DATA_Z_LSB: u8 = 0x26;
pub const BNO055_QUA_DATA_Z_MSB: u8 = 0x27;

/// Linear acceleration data
pub const BNO055_LIA_DATA_X_LSB: u8 = 0x28;
pub const BNO055_LIA_DATA_X_MSB: u8 = 0x29;
pub const BNO055_LIA_DATA_Y_LSB: u8 = 0x2A;
pub const BNO055_LIA_DATA_Y_MSB: u8 = 0x2B;
pub const BNO055_LIA_DATA_Z_LSB: u8 = 0x2C;
pub const BNO055_LIA_DATA_Z_MSB: u8 = 0x2D;

/// Gravity vector data
pub const BNO055_GRV_DATA_X_LSB: u8 = 0x2E;
pub const BNO055_GRV_DATA_X_MSB: u8 = 0x2F;
pub const BNO055_GRV_DATA_Y_LSB: u8 = 0x30;
pub const BNO055_GRV_DATA_Y_MSB: u8 = 0x31;
pub const BNO055_GRV_DATA_Z_LSB: u8 = 0x32;
pub const BNO055_GRV_DATA_Z_MSB: u8 = 0x33;

/// Temperature data
pub const BNO055_TEMP: u8 = 0x34;

/// Calibration Status
pub const BNO055_CALIB_STAT: u8 = 0x35;

pub const BNO055_ST_RESULT: u8 = 0x36;
pub const BNO055_INT_STA: u8 = 0x37;
pub const BNO055_SYS_CLK_STATUS: u8 = 0x38;
pub const BNO055_SYS_STATUS: u8 = 0x39;
pub const BNO055_SYS_ERR: u8 = 0x3A;
pub const BNO055_UNIT_SEL: u8 = 0x3B;
pub const BNO055_OPR_MODE: u8 = 0x3D;
pub const BNO055_PWR_MODE: u8 = 0x3E;

pub const BNO055_SYS_TRIGGER: u8 = 0x3F;
pub const BNO055_SYS_TRIGGER_RST_SYS_BIT: u8 = 0x20; // Reset command
pub const BNO055_SYS_TRIGGER_SELF_TEST_BIT: u8 = 0b000_0001; // Self-test command
pub const BNO055_TEMP_SOURCE: u8 = 0x40;
pub const BNO055_AXIS_MAP_CONFIG: u8 = 0x41;
pub const BNO055_AXIS_MAP_SIGN: u8 = 0x42;

/// Calibration data

pub const BNO055_ACC_OFFSET_X_LSB: u8 = 0x55;
pub const BNO055_ACC_OFFSET_X_MSB: u8 = 0x56;
pub const BNO055_ACC_OFFSET_Y_LSB: u8 = 0x57;
pub const BNO055_ACC_OFFSET_Y_MSB: u8 = 0x58;
pub const BNO055_ACC_OFFSET_Z_LSB: u8 = 0x59;
pub const BNO055_ACC_OFFSET_Z_MSB: u8 = 0x5A;

pub const BNO055_MAG_OFFSET_X_LSB: u8 = 0x5B;
pub const BNO055_MAG_OFFSET_X_MSB: u8 = 0x5C;
pub const BNO055_MAG_OFFSET_Y_LSB: u8 = 0x5D;
pub const BNO055_MAG_OFFSET_Y_MSB: u8 = 0x5E;
pub const BNO055_MAG_OFFSET_Z_LSB: u8 = 0x5F;
pub const BNO055_MAG_OFFSET_Z_MSB: u8 = 0x60;

pub const BNO055_GYR_OFFSET_X_LSB: u8 = 0x61;
pub const BNO055_GYR_OFFSET_X_MSB: u8 = 0x62;
pub const BNO055_GYR_OFFSET_Y_LSB: u8 = 0x63;
pub const BNO055_GYR_OFFSET_Y_MSB: u8 = 0x64;
pub const BNO055_GYR_OFFSET_Z_LSB: u8 = 0x65;
pub const BNO055_GYR_OFFSET_Z_MSB: u8 = 0x66;

pub const BNO055_ACC_RADIUS_LSB: u8 = 0x67;
pub const BNO055_ACC_RADIUS_MSB: u8 = 0x68;
pub const BNO055_MAG_RADIUS_LSB: u8 = 0x69;
pub const BNO055_MAG_RADIUS_MSB: u8 = 0x6A;
