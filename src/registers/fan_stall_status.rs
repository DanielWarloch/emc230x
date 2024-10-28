bitfield::bitfield! {
    /// The Fan Spin Status register indicates which fan driver has failed to spin up.
    /// All bits are Cleared upon a read if the error condition has been removed.
    #[derive(Clone, Copy)]
    pub struct FanStallStatus(u8);
    impl Debug;

    /// Fan 5 Stall Status
    ///
    /// 0: Stall has not been detected.
    ///
    /// 1: Tachometer count has exceeded maximum valid TACH count, indicating stall.
    pub f5stl, _: 4;

    /// Fan 4 Stall Status
    ///
    /// 0: Stall has not been detected.
    ///
    /// 1: Tachometer count has exceeded maximum valid TACH count, indicating stall.
    pub f4stl, _: 3;

    /// Fan 3 Stall Status
    ///
    /// 0: Stall has not been detected.
    ///
    /// 1: Tachometer count has exceeded maximum valid TACH count, indicating stall.
    pub f3stl, _: 2;

    /// Fan 2 Stall Status
    ///
    /// 0: Stall has not been detected.
    ///
    /// 1: Tachometer count has exceeded maximum valid TACH count, indicating stall.
    pub f2stl, _: 1;

    /// Fan 1 Stall Status
    ///
    /// 0: Stall has not been detected.
    ///
    /// 1: Tachometer count has exceeded maximum valid TACH count, indicating stall.
    pub f1stl, _: 0;
}

basic_from_and_into!(FanStallStatus, u8);
