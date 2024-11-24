use super::RegisterOffset;
use emc230x_macros::RegisterOffset;

bitfield::bitfield! {
    #[derive(Clone, Copy, RegisterOffset)]
    #[register(offset = 0x0E, default = 0xFF)]
    pub struct TachReadingLow(u8);
    impl Debug;

    /// Tach Reading Low Byte
    pub fxtr, set_fxtr: 7, 3;
}

bitfield::bitfield! {
    #[derive(Clone, Copy, RegisterOffset)]
    #[register(offset = 0x0F, default = 0xF8)]
    pub struct TachReadingHigh(u8);
    impl Debug;

    /// Tach Reading High Byte
    pub fxtr, set_fxtr: 7, 0;
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
        low.set_fxtr((val & 0x1F) as u8);

        let mut high = TachReadingHigh::default();
        high.set_fxtr(((val & 0x3FE0) >> 5_u8) as u8);

        Self { low, high }
    }
}

impl From<TachReading> for u16 {
    fn from(val: TachReading) -> Self {
        let low = val.low.fxtr() as u16;
        let high = val.high.fxtr() as u16;
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
