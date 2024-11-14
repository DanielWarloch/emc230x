use super::RegisterOffset;
use emc230x_macros::RegisterOffset;

bitfield::bitfield! {
    #[derive(Clone, Copy, RegisterOffset)]
    #[register(offset = 0x0C, default = 0xF8)]
    pub struct TachTargetLow(u8);
    impl Debug;

    /// Tach Target Low Byte
    pub fxtt, set_fxtt: 7, 3;
}

bitfield::bitfield! {
    #[derive(Clone, Copy, RegisterOffset)]
    #[register(offset = 0x0D, default = 0xFF)]
    pub struct TachTargetHigh(u8);
    impl Debug;

    /// Tach Target High Byte
    pub fxtt, set_fxtt: 7, 0;
}

#[derive(Clone, Copy, Debug)]
struct TachTarget {
    low: TachTargetLow,
    high: TachTargetHigh,
}

impl From<(TachTargetLow, TachTargetHigh)> for TachTarget {
    fn from(val: (TachTargetLow, TachTargetHigh)) -> Self {
        Self {
            low: val.0,
            high: val.1,
        }
    }
}

impl From<TachTarget> for (TachTargetLow, TachTargetHigh) {
    fn from(val: TachTarget) -> Self {
        (val.low, val.high)
    }
}

impl From<u16> for TachTarget {
    fn from(val: u16) -> Self {
        let mut low = TachTargetLow::default();
        low.set_fxtt((val & 0x1F) as u8);

        let mut high = TachTargetHigh::default();
        high.set_fxtt(((val & 0x3FE0) >> 5_u8) as u8);

        Self { low, high }
    }
}

impl From<TachTarget> for u16 {
    fn from(val: TachTarget) -> Self {
        let low = val.low.fxtt() as u16;
        let high = val.high.fxtt() as u16;
        (high << 5_u16) | low
    }
}

mod tests {
    #[test]
    fn tach_target() {
        let tach_target = super::TachTarget::from(0x1FFF);

        assert_eq!(tach_target.low.0, 0b1111_1000);
        assert_eq!(tach_target.high.0, 0b1111_1111);
        assert_eq!(u16::from(tach_target), 0x1FFF);
    }
}
