// Copyright (c) 2024 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::RegisterOffset;
use emc230x_macros::RegisterOffset;

bitfield::bitfield! {
    #[derive(Clone, Copy, RegisterOffset)]
    #[register(offset = 0x01, default = 0x01)]
    pub struct PwmDivide(u8);
    impl Debug;

    /// PWM Divide
    ///
    /// The final drive frequency is divided by the value of this register. The
    /// duty cycle is not affected by this setting. A value of 0x00 will be
    /// interpreted as 0x01.
    pub pwm_divide, set_pwm_divide: 7, 0;
}
