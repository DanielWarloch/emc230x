use super::RegisterOffset;
use crate::hacky_round;
use emc230x_macros::RegisterOffset;

#[derive(Copy, Clone, Debug, RegisterOffset)]
#[register(offset = 0x00, default = 0x00)]
pub struct FanDriveSetting(u8);

impl FanDriveSetting {
    pub fn duty_cycle(&self) -> u8 {
        let duty = (self.0 as f64 / 255.0) * 100.0;
        hacky_round(duty)
    }

    pub fn from_duty_cycle(duty: u8) -> Self {
        let raw = (duty as f64 / 100.0) * 255.0;
        let raw = hacky_round(raw);
        FanDriveSetting(raw)
    }
}
