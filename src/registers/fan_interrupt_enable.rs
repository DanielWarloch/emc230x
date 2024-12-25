// Copyright (c) 2024 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::RegisterAddress;
use emc230x_macros::RegisterAddress;

bitfield::bitfield! {
    #[derive(Clone, Copy, RegisterAddress)]
    #[register(address = 0x29, default = 0x00)]
    pub struct FanInterruptEnable(u8);
    impl Debug;

    /// Fan 5 Interrupt Enable
    ///
    /// Allows the fan to assert the !ALERT pin if an error condition is detected.
    ///
    /// 0: Fan is operating within limits.
    ///
    /// 1: Fan has an error condition.
    pub f5iten, _: 4;

    /// Fan 4 Interrupt Enable
    ///
    /// Allows the fan to assert the !ALERT pin if an error condition is detected.
    ///
    /// 0: Fan is operating within limits.
    ///
    /// 1: Fan has an error condition.
    pub f4iten, _: 3;

    /// Fan 3 Interrupt Enable
    ///
    /// Allows the fan to assert the !ALERT pin if an error condition is detected.
    ///
    /// 0: Fan is operating within limits.
    ///
    /// 1: Fan has an error condition.
    pub f3iten, _: 2;

    /// Fan 2 Interrupt Enable
    ///
    /// Allows the fan to assert the !ALERT pin if an error condition is detected.
    ///
    /// 0: Fan is operating within limits.
    ///
    /// 1: Fan has an error condition.
    pub f2iten, _: 1;

    /// Fan 1 Interrupt Enable
    ///
    /// Allows the fan to assert the !ALERT pin if an error condition is detected.
    ///
    /// 0: Fan is operating within limits.
    ///
    /// 1: Fan has an error condition.
    pub f1iten, _: 0;
}
