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
    #[register(offset = 0x09, default = 0xF5)]
    pub struct ValidTachCount(u8);
    impl Debug;

    /// Valid Tach Count
    pub fxvt, set_fxvt: 7, 0;
}

impl ValidTachCount {
    pub fn max_tach_count(&self) -> u16 {
        (self.0 as u16) << 5_u16
    }
}
