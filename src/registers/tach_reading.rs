use super::RegisterOffset;
use emc230x_macros::RegisterOffset;

bitfield::bitfield! {
    #[derive(Clone, Copy, Default, RegisterOffset)]
    #[register(offset = 0x0E)]
    pub struct TachReadingLow(u8);
    impl Debug;

    /// Tach Reading Low Byte
    pub fxtt, set_fxtt: 7, 3;
}

bitfield::bitfield! {
    #[derive(Clone, Copy, Default, RegisterOffset)]
    #[register(offset = 0x0F)]
    pub struct TachReadingHigh(u8);
    impl Debug;

    /// Tach Reading High Byte
    pub fxtt, set_fxtt: 7, 0;
}

#[derive(Clone, Copy, Debug)]
struct TachReading {
    low: TachReadingLow,
    high: TachReadingHigh,
}

impl From<(TachReadingLow, TachReadingHigh)> for TachReading {
    fn from(val: (TachReadingLow, TachReadingHigh)) -> Self {
        Self {
            low: val.0,
            high: val.1,
        }
    }
}

impl From<TachReading> for (TachReadingLow, TachReadingHigh) {
    fn from(val: TachReading) -> Self {
        (val.low, val.high)
    }
}

impl From<u16> for TachReading {
    fn from(val: u16) -> Self {
        let mut low = TachReadingLow::default();
        low.set_fxtt((val & 0x1F) as u8);

        let mut high = TachReadingHigh::default();
        high.set_fxtt(((val & 0x3FE0) >> 5_u8) as u8);

        Self { low, high }
    }
}

impl From<TachReading> for u16 {
    fn from(val: TachReading) -> Self {
        let low = val.low.fxtt() as u16;
        let high = val.high.fxtt() as u16;
        (high << 5_u16) | low
    }
}

mod tests {
    #[test]
    fn tach_reading() {
        let tach_reading = super::TachReading::from(0x1FFF);

        assert_eq!(tach_reading.low.0, 0b1111_1000);
        assert_eq!(tach_reading.high.0, 0b1111_1111);
        assert_eq!(u16::from(tach_reading), 0x1FFF);
    }
}
