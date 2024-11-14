use num_enum::{FromPrimitive, IntoPrimitive};

use super::RegisterAddress;
use emc230x_macros::RegisterAddress;

#[derive(Clone, Copy, Debug, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
enum PwmBaseFrequencyKhz {
    Pwm2_441 = 0b11,
    Pwm4_882 = 0b10,
    Pwm19_53 = 0b01,
    #[default]
    Pwm26_00 = 0b00,
}

bitfield::bitfield! {
    #[derive(Clone, Copy, RegisterAddress)]
    #[register(address = 0x2C, default = 0x00)]
    pub struct PwmBase45(u8);
    impl Debug;

    /// PWM5 Base Frequency
    pub u8, from into PwmBaseFrequencyKhz, pmb5, set_pmb5: 3, 2;

    /// PWM4 Base Frequency
    pub u8, from into PwmBaseFrequencyKhz, pmb4, set_pmb4: 1, 0;
}

bitfield::bitfield! {
    #[derive(Clone, Copy, RegisterAddress)]
    #[register(address = 0x2D, default = 0x00)]
    pub struct PwmBase123(u8);
    impl Debug;

    /// PWM3 Base Frequency
    pub u8, from into PwmBaseFrequencyKhz, pmb3, set_pmb3: 5, 4;

    /// PWM2 Base Frequency
    pub u8, from into PwmBaseFrequencyKhz, pmb2, set_pmb2: 3, 2;

    /// PWM1 Base Frequency
    pub u8, from into PwmBaseFrequencyKhz, pmb1, set_pmb1: 1, 0;
}
