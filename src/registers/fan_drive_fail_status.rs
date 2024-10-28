bitfield::bitfield! {
    #[derive(Clone, Copy)]
    pub struct FanDriveFailStatus(u8);
    impl Debug;

    /// Drive Fail Fan 5 Status
    ///
    /// 0: Fan is within limits of RPM.
    ///
    /// 1: Fan is unable to reach the RPM with 100% PWM input.
    pub drvf5, _: 4;

    /// Drive Fail Fan 4 Status
    ///
    /// 0: Fan is within limits of RPM.
    ///
    /// 1: Fan is unable to reach the RPM with 100% PWM input.
    pub drvf4, _: 3;

    /// Drive Fail Fan 3 Status
    ///
    /// 0: Fan is within limits of RPM.
    ///
    /// 1: Fan is unable to reach the RPM with 100% PWM input.
    pub drvf3, _: 2;

    /// Drive Fail Fan 2 Status
    ///
    /// 0: Fan is within limits of RPM.
    ///
    /// 1: Fan is unable to reach the RPM with 100% PWM input.
    pub drvf2, _: 1;

    /// Drive Fail Fan 1 Status
    ///
    /// 0: Fan is within limits of RPM.
    ///
    /// 1: Fan is unable to reach the RPM with 100% PWM input.
    pub drvf1, _: 0;
}

basic_from_and_into!(FanDriveFailStatus, u8);
