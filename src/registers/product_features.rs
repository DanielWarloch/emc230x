use super::RegisterAddress;
use emc230x_macros::RegisterAddress;

bitfield::bitfield! {
    #[derive(Clone, Copy, RegisterAddress)]
    #[register(address = 0xFC)]
    pub struct ProductFeatures(u8);
    impl Debug;

    pub u8, adr, set_adr: 5, 3;

    pub u8, fsp, set_fsp: 2, 0;
}
