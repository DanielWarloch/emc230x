use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{Error, FanSelect};

macro_rules! basic_from_and_into {
    ($name:ident, $ty:ty) => {
        impl From<$name> for $ty {
            fn from(val: $name) -> $ty {
                val.0
            }
        }

        impl From<$ty> for $name {
            fn from(val: $ty) -> $name {
                $name(val)
            }
        }
    };
}

pub(crate) mod configuration;
pub(crate) mod fan_configuration1;
pub(crate) mod fan_drive_fail_status;
pub(crate) mod fan_interrupt_enable;
pub(crate) mod fan_min_drive;
pub(crate) mod fan_spin_status;
pub(crate) mod fan_stall_status;
pub(crate) mod fan_status;
pub(crate) mod product_id;
pub(crate) mod pwm_output_config;
pub(crate) mod pwm_polarity_config;

pub(crate) const FAN1_BASE: u8 = 0x30;
pub(crate) const FAN2_BASE: u8 = 0x40;
pub(crate) const FAN3_BASE: u8 = 0x50;
pub(crate) const FAN4_BASE: u8 = 0x60;
pub(crate) const FAN5_BASE: u8 = 0x70;

pub(crate) const FAN_SETTING_OFFSET: u8 = 0;
pub(crate) const PWM_DIVIDE_OFFSET: u8 = 1;
pub(crate) const FAN_CONFIGURATION2_OFFSET: u8 = 3;
pub(crate) const GAIN_OFFSET: u8 = 5;
pub(crate) const FAN_SPIN_UP_CONFIGURATION_OFFSET: u8 = 6;
pub(crate) const FAN_MAX_STEP_OFFSET: u8 = 7;
pub(crate) const FAN_VALID_TACH_COUNT_OFFSET: u8 = 9;
pub(crate) const FAN_DRIVE_FAIL_BAND_LOW_BYTE_OFFSET: u8 = 10;
pub(crate) const FAN_DRIVE_FAIL_BAND_HIGH_BYTE_OFFSET: u8 = 11;
pub(crate) const TACH_TARGET_LOW_BYTE_OFFSET: u8 = 12;
pub(crate) const TACH_TARGET_HIGH_BYTE_OFFSET: u8 = 13;
pub(crate) const TACH_READING_HIGH_BYTE_OFFSET: u8 = 14;
pub(crate) const TACH_READ_LOW_BYTE_OFFSET: u8 = 15;

pub(crate) fn fan_register_address(sel: FanSelect, offset: u8) -> Result<Register, Error> {
    let base = match sel {
        FanSelect::Fan(fan) => match fan {
            1 => FAN1_BASE,
            2 => FAN2_BASE,
            3 => FAN3_BASE,
            4 => FAN4_BASE,
            5 => FAN5_BASE,
            _ => return Err(Error::InvalidFan),
        },
    };

    let reg: Register = (base + offset)
        .try_into()
        .map_err(|_| Error::InvalidRegister)?;

    Ok(reg)
}

#[derive(Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Register {
    Configuration = 0x20,
    FanStatus = 0x24,
    FanStallStatus = 0x25,
    FanSpinStatus = 0x26,
    DriveFailStatus = 0x27,
    FanInterruptEnable = 0x29,
    PwmPolarityConfig = 0x2A,
    PwmOutputConfig = 0x2B,
    PwmBaseF45 = 0x2C,
    PwmBaseF123 = 0x2D,
    Fan1Setting = 0x30,
    Pwm1Divide = 0x31,
    Fan1Configuration1 = 0x32,
    Fan1Configuration2 = 0x33,
    Gain1 = 0x35,
    Fan1SpinUpConfiguration = 0x36,
    Fan1MaxStep = 0x37,
    Fan1MinimumDrive = 0x38,
    Fan1ValidTachCount = 0x39,
    Fan1DriveFailBandLowByte = 0x3A,
    Fan1DriveFailBandHighByte = 0x3B,
    Tach1TargetLowByte = 0x3C,
    Tach1TargetHighByte = 0x3D,
    Tach1ReadingHighByte = 0x3E,
    Tach1ReadLowByte = 0x3F,
    Fan2Setting = 0x40,
    Pwm2Divide = 0x41,
    Fan2Configuration1 = 0x42,
    Fan2Configuration2 = 0x43,
    Gain2 = 0x45,
    Fan2SpinUpConfiguration = 0x46,
    Fan2MaxStep = 0x47,
    Fan2MinimumDrive = 0x48,
    Fan2ValidTachCount = 0x49,
    Fan2DriveFailBandLowByte = 0x4A,
    Fan2DriveFailBandHighByte = 0x4B,
    Tach2TargetLowByte = 0x4C,
    Tach2TargetHighByte = 0x4D,
    Tach2ReadingHighByte = 0x4E,
    Tach2ReadLowByte = 0x4F,
    Fan3Setting = 0x50,
    Pwm3Divide = 0x51,
    Fan3Configuration1 = 0x52,
    Fan3Configuration2 = 0x53,
    Gain3 = 0x55,
    Fan3SpinUpConfiguration = 0x56,
    Fan3MaxStep = 0x57,
    Fan3MinimumDrive = 0x58,
    Fan3ValidTachCount = 0x59,
    Fan3DriveFailBandLowByte = 0x5A,
    Fan3DriveFailBandHighByte = 0x5B,
    Tach3TargetLowByte = 0x5C,
    Tach3TargetHighByte = 0x5D,
    Tach3ReadingHighByte = 0x5E,
    Tach3ReadLowByte = 0x5F,
    Fan4Setting = 0x60,
    Pwm4Divide = 0x61,
    Fan4Configuration1 = 0x62,
    Fan4Configuration2 = 0x63,
    Gain4 = 0x65,
    Fan4SpinUpConfiguration = 0x66,
    Fan4MaxStep = 0x67,
    Fan4MinimumDrive = 0x68,
    Fan4ValidTachCount = 0x69,
    Fan4DriveFailBandLowByte = 0x6A,
    Fan4DriveFailBandHighByte = 0x6B,
    Tach4TargetLowByte = 0x6C,
    Tach4TargetHighByte = 0x6D,
    Tach4ReadingHighByte = 0x6E,
    Tach4ReadLowByte = 0x6F,
    Fan5Setting = 0x70,
    Pwm5Divide = 0x71,
    Fan5Configuration1 = 0x72,
    Fan5Configuration2 = 0x73,
    Gain5 = 0x75,
    Fan5SpinUpConfiguration = 0x76,
    Fan5MaxStep = 0x77,
    Fan5MinimumDrive = 0x78,
    Fan5ValidTachCount = 0x79,
    Fan5DriveFailBandLowByte = 0x7A,
    Fan5DriveFailBandHighByte = 0x7B,
    Tach5TargetLowByte = 0x7C,
    Tach5TargetHighByte = 0x7D,
    Tach5ReadingHighByte = 0x7E,
    Tach5ReadLowByte = 0x7F,
    SoftwareLock = 0xEF,
    ProductFeatures = 0xFC,
    ProductId = 0xFD,
    ManufacturerId = 0xFE,
    Revision = 0xFF,
}
