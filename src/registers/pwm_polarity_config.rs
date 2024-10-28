bitfield::bitfield! {
    #[derive(Clone, Copy)]
    pub struct PwmPolarityConfig(u8);
    impl Debug;

    /// Fan 5 PWM Output Polarity
    ///
    /// 0: Fan drive setting of 0x00 produces 0% duty cycle. Fan drive setting of 0xFF produces
    ///    100% duty cycle.
    ///
    /// 1: Fan drive setting of 0x00 produces 100% duty cycle. Fan drive setting of 0xFF produces
    ///    0% duty cycle.
    pub plrity5, set_plrity5: 4;

    /// Fan 4 PWM Output Polarity
    ///
    /// 0: Fan drive setting of 0x00 produces 0% duty cycle. Fan drive setting of 0xFF produces
    ///    100% duty cycle.
    ///
    /// 1: Fan drive setting of 0x00 produces 100% duty cycle. Fan drive setting of 0xFF produces
    ///    0% duty cycle.
    pub plrity4, set_plrity4: 3;

    /// Fan 3 PWM Output Polarity
    ///
    /// 0: Fan drive setting of 0x00 produces 0% duty cycle. Fan drive setting of 0xFF produces
    ///    100% duty cycle.
    ///
    /// 1: Fan drive setting of 0x00 produces 100% duty cycle. Fan drive setting of 0xFF produces
    ///    0% duty cycle.
    pub plrity3, set_plrity3: 2;

    /// Fan 2 PWM Output Polarity
    ///
    /// 0: Fan drive setting of 0x00 produces 0% duty cycle. Fan drive setting of 0xFF produces
    ///    100% duty cycle.
    ///
    /// 1: Fan drive setting of 0x00 produces 100% duty cycle. Fan drive setting of 0xFF produces
    ///    0% duty cycle.
    pub plrity2, set_plrity2: 1;

    /// Fan 1 PWM Output Polarity
    ///
    /// 0: Fan drive setting of 0x00 produces 0% duty cycle. Fan drive setting of 0xFF produces
    ///    100% duty cycle.
    ///
    /// 1: Fan drive setting of 0x00 produces 100% duty cycle. Fan drive setting of 0xFF produces
    ///    0% duty cycle.
    pub plrity1, set_plrity1: 0;
}

basic_from_and_into!(PwmPolarityConfig, u8);
