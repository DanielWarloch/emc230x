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
    #[register(address = 0xEF, default = 0xF8)]
    pub struct SoftwareLock(u8);
    impl Debug;

    /// Software Lock
    ///
    /// 0: All SWL registers are writable.
    ///
    /// 1: All SWL registers are read-only. Unlock occurs on power cycle.
    pub u8, lock, set_lock: 0;
}
