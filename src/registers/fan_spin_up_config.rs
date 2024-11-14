use num_enum::{FromPrimitive, IntoPrimitive};

use super::RegisterOffset;
use emc230x_macros::RegisterOffset;

bitfield::bitfield! {
    #[derive(Clone, Copy, RegisterOffset)]
    #[register(offset = 0x06, default = 0x19)]
    pub struct FanSpinUpConfig(u8);
    impl Debug;

    /// Drive Fail Count
    ///
    /// Determines when an aging fan has been detected.
    pub u8, from into DriveFailCount, dfcx, set_dfcx: 7, 6;

    /// No Kick
    ///
    /// Drive to 100% duty cycle for 1/4 of the spin-up time.
    ///
    /// 0: Spin-Up will drive to 100%
    ///
    /// 1: Spin-Up will not drive to 100%
    pub nkckx, set_nkckx: 5;

    /// Spin-Up Level
    pub u8, from into SpinUpLevel, splvx, set_splvx: 4, 2;

    /// Spin-Up Time
    ///
    /// How long the spin-up routine will run before releasing the drive.
    pub u8, from into SpinUpTimeMs, spltx, set_spltx: 1, 0;
}

#[derive(Clone, Copy, Debug, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum DriveFailCount {
    #[default]
    Disabled = 0b00,
    UpdatePeriod16Ms = 0b01,
    UpdatePeriod32Ms = 0b10,
    UpdatePeriod64Ms = 0b11,
}

#[derive(Clone, Copy, Debug, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum SpinUpLevel {
    Level30 = 0b000,
    Level35 = 0b001,
    Level40 = 0b010,
    Level45 = 0b011,
    Level50 = 0b100,
    Level55 = 0b101,
    #[default]
    Level60 = 0b110,
    Level65 = 0b111,
}

#[derive(Clone, Copy, Debug, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum SpinUpTimeMs {
    Time250 = 0b00,
    #[default]
    Time500 = 0b01,
    Time1000 = 0b10,
    Time2000 = 0b11,
}
