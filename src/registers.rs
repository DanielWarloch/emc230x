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

pub(crate) mod configuration {
    bitfield::bitfield! {
        /// The Configuration register controls the basic functionality of the device.
        /// The Configuration regsister is software locked.
        #[derive(Clone, Copy)]
        pub struct Configuration(u8);
        impl Debug;

        /// Blocks the !ALERT pin from asserting.
        ///
        /// 0: Alert functionality is enabled. If any bit in the status register is
        ///    set, the !ALERT pin will be asserted.
        ///
        /// 1: Alert functionality is disabled.
        pub mask, set_mask: 7;

        /// Disable SMBus Time-Out function.
        ///
        /// 0: SMBus Time-Out function is enabled.
        ///
        /// 1: SMBus Time-Out function is disabled. The device will be fully I2C compliant.
        pub dis_to, set_dis_to: 6;

        /// Enable the Watchdog Timer.
        ///
        /// 0: Watchdog Timer is disabled. Runs once on power-up and then is disabled.
        ///
        /// 1: Watchdog Timer operates continuously.
        pub wd_en, set_wd_en: 5;

        /// Tachometer Clock Direction
        ///
        /// Allows the internal tachometer clock to be driven out of the CLK pin so that
        /// multiple devices can be synchronized to the same source.
        ///
        /// 0: The CLK pin acts as a clock input.
        ///
        /// 1: The CLK pin acts as a clock output (push-pull).
        pub dreck, set_dreck: 1;

        /// Tachometer Clock Selection
        ///
        /// Enables the device to use the CLK pin as the tachometer clock. If DRECK is set,
        /// this bit is ignored and the device will use the internal oscillator.
        ///
        /// 0: The device uses the internal oscillator.
        ///
        /// 1: The device uses the CLK pin as the tachometer clock.
        pub useck, set_useck: 0;
    }

    basic_from_and_into!(Configuration, u8);
}

pub(crate) mod fan_status {
    bitfield::bitfield! {
        /// The Fan Status register indicates that the fan driver has stalled, failed, or
        /// the Watchdog Timer has expired.
        #[derive(Clone, Copy)]
        pub struct FanStatus(u8);
        impl Debug;

        /// Watchdog Timer Status
        ///
        /// When the bit is set, each fan is driven to 100% duty cycle until they are
        /// programmed. The bit is cleared when it is read.
        ///
        /// 0: Watchdog Timer has not expired.
        ///
        /// 1: Watchdog Timer has expired.
        pub watch, _: 7;

        /// Drive Fail Status
        ///
        /// Indicates that one or more fan drivers cannot meet the programmed fan speed at
        /// maximum duty cycle.
        ///
        /// 0: All bits in Fan Drive Fail Status register are clear.
        ///
        /// 1: Any bit in the Fan Drive Fail Status register is set.
        pub dvfail, _: 2;

        /// Fan Spin Status
        ///
        /// Indicates that one or more fan drivers cannot spin up.
        ///
        /// 0: All bits in the Fan Spin Status register are clear.
        ///
        /// 1: Any bit in the Fan Spin Status register is set.
        pub fnspin, _: 1;

        /// Fan Stall Status
        ///
        /// Indicates that one or more fan drivers are stalled.
        ///
        /// 0: All bits in the Fan Stall Status register are clear.
        ///
        /// 1: Any bit in the Fan Stall Status register is set.
        pub fnstl, _: 0;
    }

    basic_from_and_into!(FanStatus, u8);
}

pub(crate) mod fan_stall_status {
    bitfield::bitfield! {
        /// The Fan Spin Status register indicates which fan driver has failed to spin up.
        /// All bits are Cleared upon a read if the error condition has been removed.
        #[derive(Clone, Copy)]
        pub struct FanStallStatus(u8);
        impl Debug;

        /// Fan 5 Stall Status
        ///
        /// 0: Stall has not been detected.
        ///
        /// 1: Tachometer count has exceeded maximum valid TACH count, indicating stall.
        pub f5stl, _: 4;

        /// Fan 4 Stall Status
        ///
        /// 0: Stall has not been detected.
        ///
        /// 1: Tachometer count has exceeded maximum valid TACH count, indicating stall.
        pub f4stl, _: 3;

        /// Fan 3 Stall Status
        ///
        /// 0: Stall has not been detected.
        ///
        /// 1: Tachometer count has exceeded maximum valid TACH count, indicating stall.
        pub f3stl, _: 2;

        /// Fan 2 Stall Status
        ///
        /// 0: Stall has not been detected.
        ///
        /// 1: Tachometer count has exceeded maximum valid TACH count, indicating stall.
        pub f2stl, _: 1;

        /// Fan 1 Stall Status
        ///
        /// 0: Stall has not been detected.
        ///
        /// 1: Tachometer count has exceeded maximum valid TACH count, indicating stall.
        pub f1stl, _: 0;
    }

    basic_from_and_into!(FanStallStatus, u8);
}

pub(crate) mod fan_spin_status {
    bitfield::bitfield! {
        #[derive(Clone, Copy)]
        pub struct FanSpinStatus(u8);
        impl Debug;

        /// Fan 5 Spin Status
        ///
        /// 0: Spin-up routine was successful.
        ///
        /// 1: Spin-up routine failed to start the fun.
        pub f5spin, _: 4;

        /// Fan 4 Spin Status
        ///
        /// 0: Spin-up routine was successful.
        ///
        /// 1: Spin-up routine failed to start the fun.
        pub f4spin, _: 3;

        /// Fan 3 Spin Status
        ///
        /// 0: Spin-up routine was successful.
        ///
        /// 1: Spin-up routine failed to start the fun.
        pub f3spin, _: 2;

        /// Fan 2 Spin Status
        ///
        /// 0: Spin-up routine was successful.
        ///
        /// 1: Spin-up routine failed to start the fun.
        pub f2spin, _: 1;

        /// Fan 1 Spin Status
        ///
        /// 0: Spin-up routine was successful.
        ///
        /// 1: Spin-up routine failed to start the fun.
        pub f1spin, _: 0;
    }

    basic_from_and_into!(FanSpinStatus, u8);
}

pub(crate) mod fan_drive_fail_status {
    bitfield::bitfield! {
        #[derive(Clone, Copy)]
        pub struct FanDriveFailStatus(u8);
        impl Debug;

        /// Drive Fail Fan 5 Status
        ///
        /// 0: Fan is within limits of RPM.
        ///
        /// 1: Fan is unable to reach the RPM with 100% PWM input.
        pub drvf5, _: 4;

        /// Drive Fail Fan 4 Status
        ///
        /// 0: Fan is within limits of RPM.
        ///
        /// 1: Fan is unable to reach the RPM with 100% PWM input.
        pub drvf4, _: 3;

        /// Drive Fail Fan 3 Status
        ///
        /// 0: Fan is within limits of RPM.
        ///
        /// 1: Fan is unable to reach the RPM with 100% PWM input.
        pub drvf3, _: 2;

        /// Drive Fail Fan 2 Status
        ///
        /// 0: Fan is within limits of RPM.
        ///
        /// 1: Fan is unable to reach the RPM with 100% PWM input.
        pub drvf2, _: 1;

        /// Drive Fail Fan 1 Status
        ///
        /// 0: Fan is within limits of RPM.
        ///
        /// 1: Fan is unable to reach the RPM with 100% PWM input.
        pub drvf1, _: 0;
    }

    basic_from_and_into!(FanDriveFailStatus, u8);
}

pub(crate) mod fan_interrupt_enable {
    bitfield::bitfield! {
        #[derive(Clone, Copy)]
        pub struct FanInterruptEnable(u8);
        impl Debug;

        /// Fan 5 Interrupt Enable
        ///
        /// Allows the fan to assert the !ALERT pin if an error condition is detected.
        ///
        /// 0: Fan is operating within limits.
        ///
        /// 1: Fan has an error condition.
        pub f5iten, _: 4;

        /// Fan 4 Interrupt Enable
        ///
        /// Allows the fan to assert the !ALERT pin if an error condition is detected.
        ///
        /// 0: Fan is operating within limits.
        ///
        /// 1: Fan has an error condition.
        pub f4iten, _: 3;

        /// Fan 3 Interrupt Enable
        ///
        /// Allows the fan to assert the !ALERT pin if an error condition is detected.
        ///
        /// 0: Fan is operating within limits.
        ///
        /// 1: Fan has an error condition.
        pub f3iten, _: 2;

        /// Fan 2 Interrupt Enable
        ///
        /// Allows the fan to assert the !ALERT pin if an error condition is detected.
        ///
        /// 0: Fan is operating within limits.
        ///
        /// 1: Fan has an error condition.
        pub f2iten, _: 1;

        /// Fan 1 Interrupt Enable
        ///
        /// Allows the fan to assert the !ALERT pin if an error condition is detected.
        ///
        /// 0: Fan is operating within limits.
        ///
        /// 1: Fan has an error condition.
        pub f1iten, _: 0;
    }

    basic_from_and_into!(FanInterruptEnable, u8);
}

pub(crate) mod pwm_polarity_config {
    bitfield::bitfield! {
        #[derive(Clone, Copy)]
        pub struct PwmPolarityConfig(u8);
        impl Debug;

        /// Fan 5 PWM Output Polarity
        ///
        /// 0: Fan drive setting of 0x00 produces 0% duty cycle. Fan drive setting of 0xFF produces
        ///    100% duty cycle.
        ///
        /// 1: Fan drive setting of 0x00 produces 100% duty cycle. Fan drive setting of 0xFF produces
        ///    0% duty cycle.
        pub plrity5, set_plrity5: 4;

        /// Fan 4 PWM Output Polarity
        ///
        /// 0: Fan drive setting of 0x00 produces 0% duty cycle. Fan drive setting of 0xFF produces
        ///    100% duty cycle.
        ///
        /// 1: Fan drive setting of 0x00 produces 100% duty cycle. Fan drive setting of 0xFF produces
        ///    0% duty cycle.
        pub plrity4, set_plrity4: 3;

        /// Fan 3 PWM Output Polarity
        ///
        /// 0: Fan drive setting of 0x00 produces 0% duty cycle. Fan drive setting of 0xFF produces
        ///    100% duty cycle.
        ///
        /// 1: Fan drive setting of 0x00 produces 100% duty cycle. Fan drive setting of 0xFF produces
        ///    0% duty cycle.
        pub plrity3, set_plrity3: 2;

        /// Fan 2 PWM Output Polarity
        ///
        /// 0: Fan drive setting of 0x00 produces 0% duty cycle. Fan drive setting of 0xFF produces
        ///    100% duty cycle.
        ///
        /// 1: Fan drive setting of 0x00 produces 100% duty cycle. Fan drive setting of 0xFF produces
        ///    0% duty cycle.
        pub plrity2, set_plrity2: 1;

        /// Fan 1 PWM Output Polarity
        ///
        /// 0: Fan drive setting of 0x00 produces 0% duty cycle. Fan drive setting of 0xFF produces
        ///    100% duty cycle.
        ///
        /// 1: Fan drive setting of 0x00 produces 100% duty cycle. Fan drive setting of 0xFF produces
        ///    0% duty cycle.
        pub plrity1, set_plrity1: 0;
    }

    basic_from_and_into!(PwmPolarityConfig, u8);
}

pub(crate) mod fan_configuration1 {
    use num_enum::{FromPrimitive, IntoPrimitive};

    pub(crate) const OFFSET: u8 = 2;

    bitfield::bitfield! {
        #[derive(Clone, Copy)]
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
        pub u8, from into UpdateTime, udtx, set_udtx: 2, 0;
    }

    basic_from_and_into!(FanConfiguration1, u8);

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
        Sample3 = 0b00,
        #[default]
        Sample5 = 0b01,
        Sample7 = 0b10,
        Sample9 = 0b11,
    }

    impl Edges {
        pub fn num_edges(&self) -> u8 {
            match self {
                Edges::Sample3 => 3,
                Edges::Sample5 => 5,
                Edges::Sample7 => 7,
                Edges::Sample9 => 9,
            }
        }

        pub fn poles(&self) -> u8 {
            match self {
                Edges::Sample9 => 4,
                Edges::Sample7 => 3,
                Edges::Sample5 => 2,
                Edges::Sample3 => 1,
            }
        }

        pub fn tach_multiplier(&self) -> f64 {
            match self {
                Edges::Sample9 => 2.0,
                Edges::Sample7 => 1.5,
                Edges::Sample5 => 1.0,
                Edges::Sample3 => 0.5,
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

pub(crate) mod fan_min_drive {
    use crate::hacky_round;
    use bitfield::bitfield;

    pub(crate) const OFFSET: u8 = 8;

    bitfield! {
        pub struct FanMinimumDrive(u8);
        impl Debug;

        /// Minimum Drive
        ///
        /// The minimum PWM duty cycle that the device will output to the fan.
        pub min_drive, set_min_drive: 7, 0;
    }

    impl FanMinimumDrive {
        pub fn duty_cycle(&self) -> u8 {
            let duty = (self.0 as f64 / 255.0) * 100.0;
            hacky_round(duty)
        }

        pub fn from_duty_cycle(duty: u8) -> Self {
            let raw = (duty as f64 / 100.0) * 255.0;
            let raw = hacky_round(raw);
            FanMinimumDrive(raw)
        }
    }

    basic_from_and_into!(FanMinimumDrive, u8);
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
