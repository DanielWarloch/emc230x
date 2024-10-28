use bitfield::bitfield;

use super::RegisterOffset;
use crate::hacky_round;
use emc230x_macros::RegisterOffset;

bitfield! {
    #[derive(Clone, Copy, RegisterOffset)]
    #[register(offset = 0x08)]
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
