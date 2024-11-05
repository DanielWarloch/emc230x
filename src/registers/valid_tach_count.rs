use super::RegisterOffset;
use emc230x_macros::RegisterOffset;

bitfield::bitfield! {
    #[derive(Clone, Copy, RegisterOffset)]
    #[register(offset = 0x09)]
    pub struct ValidTachCount(u8);
    impl Debug;

    /// Valid Tach Count
    pub fxvt, set_fxvt: 7, 0;
}

impl ValidTachCount {
    pub fn max_tach_count(&self) -> u16 {
        (self.0 as u16) << 5_u16
    }
}
