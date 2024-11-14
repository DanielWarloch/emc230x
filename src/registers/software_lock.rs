use super::RegisterAddress;
use emc230x_macros::RegisterAddress;

bitfield::bitfield! {
    #[derive(Clone, Copy, RegisterAddress)]
    #[register(address = 0xEF, default = 0xF8)]
    pub struct SoftwareLock(u8);
    impl Debug;

    /// Software Lock
    ///
    /// 0: All SWL registers are writable.
    ///
    /// 1: All SWL registers are read-only. Unlock occurs on power cycle.
    pub u8, lock, set_lock: 0;
}
