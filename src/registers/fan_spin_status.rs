bitfield::bitfield! {
    #[derive(Clone, Copy)]
    pub struct FanSpinStatus(u8);
    impl Debug;

    /// Fan 5 Spin Status
    ///
    /// 0: Spin-up routine was successful.
    ///
    /// 1: Spin-up routine failed to start the fun.
    pub f5spin, _: 4;

    /// Fan 4 Spin Status
    ///
    /// 0: Spin-up routine was successful.
    ///
    /// 1: Spin-up routine failed to start the fun.
    pub f4spin, _: 3;

    /// Fan 3 Spin Status
    ///
    /// 0: Spin-up routine was successful.
    ///
    /// 1: Spin-up routine failed to start the fun.
    pub f3spin, _: 2;

    /// Fan 2 Spin Status
    ///
    /// 0: Spin-up routine was successful.
    ///
    /// 1: Spin-up routine failed to start the fun.
    pub f2spin, _: 1;

    /// Fan 1 Spin Status
    ///
    /// 0: Spin-up routine was successful.
    ///
    /// 1: Spin-up routine failed to start the fun.
    pub f1spin, _: 0;
}

basic_from_and_into!(FanSpinStatus, u8);
