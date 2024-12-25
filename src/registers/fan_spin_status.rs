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
    #[register(address = 0x26, default = 0x00)]
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
