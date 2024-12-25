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
    #[register(offset = 0x07, default = 0x10)]
    pub struct MaxStepSize(u8);
    impl Debug;

    /// Maximum Step Size
    ///
    /// The maximum step size that the device will use to adjust the fan speed.
    pub u8, stpx, set_stpx: 7, 0;
}
