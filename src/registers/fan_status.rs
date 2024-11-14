use super::RegisterAddress;
use emc230x_macros::RegisterAddress;

bitfield::bitfield! {
    /// The Fan Status register indicates that the fan driver has stalled, failed, or
    /// the Watchdog Timer has expired.
    #[derive(Clone, Copy, RegisterAddress)]
    #[register(address = 0x24, default = 0x00)]
    pub struct FanStatus(u8);
    impl Debug;

    /// Watchdog Timer Status
    ///
    /// When the bit is set, each fan is driven to 100% duty cycle until they are
    /// programmed. The bit is cleared when it is read.
    ///
    /// 0: Watchdog Timer has not expired.
    ///
    /// 1: Watchdog Timer has expired.
    pub watch, _: 7;

    /// Drive Fail Status
    ///
    /// Indicates that one or more fan drivers cannot meet the programmed fan speed at
    /// maximum duty cycle.
    ///
    /// 0: All bits in Fan Drive Fail Status register are clear.
    ///
    /// 1: Any bit in the Fan Drive Fail Status register is set.
    pub dvfail, _: 2;

    /// Fan Spin Status
    ///
    /// Indicates that one or more fan drivers cannot spin up.
    ///
    /// 0: All bits in the Fan Spin Status register are clear.
    ///
    /// 1: Any bit in the Fan Spin Status register is set.
    pub fnspin, _: 1;

    /// Fan Stall Status
    ///
    /// Indicates that one or more fan drivers are stalled.
    ///
    /// 0: All bits in the Fan Stall Status register are clear.
    ///
    /// 1: Any bit in the Fan Stall Status register is set.
    pub fnstl, _: 0;
}
