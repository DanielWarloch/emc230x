// Copyright (c) 2024 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::RegisterAddress;
use emc230x_macros::RegisterAddress;

bitfield::bitfield! {
    /// The Configuration register controls the basic functionality of the device.
    /// The Configuration regsister is software locked.
    #[derive(Clone, Copy, RegisterAddress)]
    #[register(address = 0x20, default = 0x40)]
    pub struct Configuration(u8);
    impl Debug;

    /// Blocks the !ALERT pin from asserting.
    ///
    /// 0: Alert functionality is enabled. If any bit in the status register is
    ///    set, the !ALERT pin will be asserted.
    ///
    /// 1: Alert functionality is disabled.
    pub mask, set_mask: 7;

    /// Disable SMBus Time-Out function.
    ///
    /// 0: SMBus Time-Out function is enabled.
    ///
    /// 1: SMBus Time-Out function is disabled. The device will be fully I2C compliant.
    pub dis_to, set_dis_to: 6;

    /// Enable the Watchdog Timer.
    ///
    /// 0: Watchdog Timer is disabled. Runs once on power-up and then is disabled.
    ///
    /// 1: Watchdog Timer operates continuously.
    pub wd_en, set_wd_en: 5;

    /// Tachometer Clock Direction
    ///
    /// Allows the internal tachometer clock to be driven out of the CLK pin so that
    /// multiple devices can be synchronized to the same source.
    ///
    /// 0: The CLK pin acts as a clock input.
    ///
    /// 1: The CLK pin acts as a clock output (push-pull).
    pub dreck, set_dreck: 1;

    /// Tachometer Clock Selection
    ///
    /// Enables the device to use the CLK pin as the tachometer clock. If DRECK is set,
    /// this bit is ignored and the device will use the internal oscillator.
    ///
    /// 0: The device uses the internal oscillator.
    ///
    /// 1: The device uses the CLK pin as the tachometer clock.
    pub useck, set_useck: 0;
}
