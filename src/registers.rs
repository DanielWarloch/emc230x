use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{Error, FanSelect};

pub(crate) const FAN1_BASE: u8 = 0x30;
pub(crate) const FAN2_BASE: u8 = 0x40;
pub(crate) const FAN3_BASE: u8 = 0x50;
pub(crate) const FAN4_BASE: u8 = 0x60;
pub(crate) const FAN5_BASE: u8 = 0x70;

pub(crate) const FAN_SETTING_OFFSET: u8 = 0;
pub(crate) const PWM_DIVIDE_OFFSET: u8 = 1;
pub(crate) const FAN_CONFIGURATION1_OFFSET: u8 = 2;
pub(crate) const FAN_CONFIGURATION2_OFFSET: u8 = 3;
pub(crate) const GAIN_OFFSET: u8 = 5;
pub(crate) const FAN_SPIN_UP_CONFIGURATION_OFFSET: u8 = 6;
pub(crate) const FAN_MAX_STEP_OFFSET: u8 = 7;
pub(crate) const FAN_MINIMUM_DRIVE_OFFSET: u8 = 8;
pub(crate) const FAN_VALID_TACH_COUNT_OFFSET: u8 = 9;
pub(crate) const FAN_DRIVE_FAIL_BAND_LOW_BYTE_OFFSET: u8 = 10;
pub(crate) const FAN_DRIVE_FAIL_BAND_HIGH_BYTE_OFFSET: u8 = 11;
pub(crate) const TACH_TARGET_LOW_BYTE_OFFSET: u8 = 12;
pub(crate) const TACH_TARGET_HIGH_BYTE_OFFSET: u8 = 13;
pub(crate) const TACH_READING_HIGH_BYTE_OFFSET: u8 = 14;
pub(crate) const TACH_READ_LOW_BYTE_OFFSET: u8 = 15;

mod fan_configuration1 {
    use num_enum::{FromPrimitive, IntoPrimitive};

    bitfield::bitfield! {
        pub struct FanConfiguration1(u8);
        impl Debug;
        /// Closed loop algorithm control
        ///
        /// 1: Closed loop algorithm is enabled. Changes to Fan Setting register are ignored.
        ///
        /// 0: Closed loop algorithm is disabled. The device is placed in Direct Setting mode.
        ///    Changes to the Fan Setting register will change the PWM duty cycle.
        pub enagx, set_enagx: 7;

        /// RPM Range
        ///
        /// The minimum fan speed measured and reported by the device.
        pub u8, from into Range, rngx, set_rngx: 6, 5;

        /// The number of edges to sample when calculating RPM.
        pub u8, from into Edges, edgx, set_edgx: 4, 3;

        /// Update Time.
        ///
        /// The PID update rate for closed loop control. The Update Time, along with
        /// the Fan Step register, is used to control the ramp rate of the drive response
        /// to provide a cleaner transition of the actual fan operation as the desired
        /// fan speed changes.
        pub udtx, set_udtx: 2, 0;
    }

    pub(crate) const OFFSET: u8 = 2;

    #[derive(Clone, Copy, Debug, FromPrimitive, IntoPrimitive)]
    #[repr(u8)]
    pub enum Range {
        Rpm500 = 0b00,
        #[default]
        Rpm1000 = 0b01,
        Rpm2000 = 0b10,
        Rpm4000 = 0b11,
    }

    impl Range {
        pub fn min_rpm(&self) -> u16 {
            match self {
                Range::Rpm500 => 500,
                Range::Rpm1000 => 1000,
                Range::Rpm2000 => 2000,
                Range::Rpm4000 => 4000,
            }
        }

        pub fn tach_count_multiplier(&self) -> u8 {
            match self {
                Range::Rpm500 => 1,
                Range::Rpm1000 => 2,
                Range::Rpm2000 => 4,
                Range::Rpm4000 => 8,
            }
        }
    }

    #[derive(Clone, Copy, Debug, FromPrimitive, IntoPrimitive)]
    #[repr(u8)]
    pub enum Edges {
        Sample3Edges = 0b00,
        #[default]
        Sample5Edges = 0b01,
        Sample7Edges = 0b10,
        Sample9Edges = 0b11,
    }

    impl Edges {
        pub fn poles(&self) -> u8 {
            match self {
                Edges::Sample9Edges => 4,
                Edges::Sample7Edges => 3,
                Edges::Sample5Edges => 2,
                Edges::Sample3Edges => 1,
            }
        }

        pub fn tach_multiplier(&self) -> f64 {
            match self {
                Edges::Sample9Edges => 2.0,
                Edges::Sample7Edges => 1.5,
                Edges::Sample5Edges => 1.0,
                Edges::Sample3Edges => 0.5,
            }
        }
    }

    #[derive(Clone, Copy, Debug, FromPrimitive, IntoPrimitive)]
    #[repr(u8)]
    pub enum UpdateTime {
        UpdateTime100ms = 0b000,
        UpdateTime200ms = 0b001,
        UpdateTime300ms = 0b010,
        #[default]
        UpdateTime400ms = 0b011,
        UpdateTime500ms = 0b100,
        UpdateTime800ms = 0b101,
        UpdateTime1200ms = 0b110,
        UpdateTime1600ms = 0b111,
    }

    impl UpdateTime {
        pub fn millis(&self) -> u16 {
            match self {
                UpdateTime::UpdateTime100ms => 100,
                UpdateTime::UpdateTime200ms => 200,
                UpdateTime::UpdateTime300ms => 300,
                UpdateTime::UpdateTime400ms => 400,
                UpdateTime::UpdateTime500ms => 500,
                UpdateTime::UpdateTime800ms => 800,
                UpdateTime::UpdateTime1200ms => 1200,
                UpdateTime::UpdateTime1600ms => 1600,
            }
        }
    }
}

#[derive(IntoPrimitive, TryFromPrimitive)]
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

#[derive(Copy, Clone, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ProductId {
    Emc2305 = 0x34,
    Emc2303 = 0x35,
    Emc2302 = 0x36,
    Emc2301 = 0x37,
}

impl ProductId {
    /// Number of fans the device supports based on the Product ID.
    pub fn num_fans(&self) -> u8 {
        match self {
            ProductId::Emc2301 => 1,
            ProductId::Emc2302 => 2,
            ProductId::Emc2303 => 3,
            ProductId::Emc2305 => 5,
        }
    }
}

impl defmt::Format for ProductId {
    fn format(&self, f: defmt::Formatter) {
        match self {
            ProductId::Emc2305 => defmt::write!(f, "EMC2305"),
            ProductId::Emc2303 => defmt::write!(f, "EMC2303"),
            ProductId::Emc2302 => defmt::write!(f, "EMC2302"),
            ProductId::Emc2301 => defmt::write!(f, "EMC2301"),
        }
    }
}
