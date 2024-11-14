use num_enum::{FromPrimitive, IntoPrimitive};

use super::RegisterOffset;
use emc230x_macros::RegisterOffset;

bitfield::bitfield! {
    #[derive(Clone, Copy, RegisterOffset)]
    #[register(offset = 0x05, default = 0x2A)]
    pub struct PidGain(u8);
    impl Debug;

    /// Derivative Gain
    ///
    /// The effective gain applied to K_D
    pub gdex, set_gdex: 5, 4;

    /// Integral Gain
    ///
    /// The effective gain applied to K_I
    pub ginx, set_ginx: 3, 2;

    /// Proportional Gain
    ///
    /// The effective gain applied to K_P
    pub gprx, set_gprx: 1, 0;
}

#[derive(Clone, Copy, Debug, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub(crate) enum PidGainMultiplier {
    X1 = 0b00,
    X2 = 0b01,
    #[default]
    X4 = 0b10,
    X8 = 0b11,
}
