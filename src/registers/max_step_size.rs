use super::RegisterOffset;
use emc230x_macros::RegisterOffset;

bitfield::bitfield! {
    #[derive(Clone, Copy, RegisterOffset)]
    #[register(offset = 0x07, default = 0x10)]
    pub struct MaxStepSize(u8);
    impl Debug;

    /// Maximum Step Size
    ///
    /// The maximum step size that the device will use to adjust the fan speed.
    pub u8, stpx, set_stpx: 7, 0;
}
