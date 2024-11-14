use super::RegisterOffset;
use emc230x_macros::RegisterOffset;

bitfield::bitfield! {
    #[derive(Clone, Copy, RegisterOffset)]
    #[register(offset = 0x0A, default = 0x00)]
    pub struct DriveFailBandLow(u8);
    impl Debug;

    /// Drive Fail Band Low Byte
    pub fxdf, set_fxdf: 7, 3;
}

bitfield::bitfield! {
    #[derive(Clone, Copy, RegisterOffset)]
    #[register(offset = 0x0B, default = 0x00)]
    pub struct DriveFailBandHigh(u8);
    impl Debug;

    /// Drive Fail Band High Byte
    pub fxdf, set_fxdf: 7, 0;
}

#[derive(Clone, Copy, Debug)]
struct DriveFailBand {
    low: DriveFailBandLow,
    high: DriveFailBandHigh,
}

impl From<(DriveFailBandLow, DriveFailBandHigh)> for DriveFailBand {
    fn from(val: (DriveFailBandLow, DriveFailBandHigh)) -> Self {
        Self {
            low: val.0,
            high: val.1,
        }
    }
}

impl From<DriveFailBand> for (DriveFailBandLow, DriveFailBandHigh) {
    fn from(val: DriveFailBand) -> Self {
        (val.low, val.high)
    }
}

impl From<u16> for DriveFailBand {
    fn from(val: u16) -> Self {
        let mut low = DriveFailBandLow::default();
        low.set_fxdf((val & 0x1F) as u8);

        let mut high = DriveFailBandHigh::default();
        high.set_fxdf(((val & 0x3FE0) >> 5_u8) as u8);

        Self { low, high }
    }
}

impl From<DriveFailBand> for u16 {
    fn from(val: DriveFailBand) -> Self {
        let low = val.low.fxdf() as u16;
        let high = val.high.fxdf() as u16;
        (high << 5_u16) | low
    }
}

mod tests {
    #[test]
    fn drive_fail_band() {
        let drive_fail_band = super::DriveFailBand::from(0x1FFF);

        assert_eq!(drive_fail_band.low.0, 0b1111_1000);
        assert_eq!(drive_fail_band.high.0, 0b1111_1111);
        assert_eq!(u16::from(drive_fail_band), 0x1FFF);
    }
}
