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
    #[register(address = 0x27, default = 0x00)]
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
