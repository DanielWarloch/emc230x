// Copyright (c) 2024 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use num_enum::{FromPrimitive, IntoPrimitive};

use super::RegisterOffset;
use emc230x_macros::RegisterOffset;

bitfield::bitfield! {
    #[derive(Clone, Copy, RegisterOffset)]
    #[register(offset = 0x03, default = 0x28)]
    pub struct FanConfiguration2(u8);
    impl Debug;

    /// Ramp Rate Control
    ///
    /// 0: Ramp Rate Control disabled.
    ///
    /// 1: Ramp Rate Control enabled.
    pub enrcx, set_enrcx: 6;

    /// Glitch Filter Enable
    ///
    /// Enables the noise filter that removes high-frequency noise injected on
    /// the TACH pin.
    ///
    /// 0: Glitch filter disabled.
    ///
    /// 1: Glitch filter enabled.
    pub ghenx, set_ghenx: 5;

    /// Derivative Options
    ///
    /// Determines what form of derivative will be used in the Fan Speed Setting
    /// calculation.
    pub u8, from into DerivativeOptions, dptx, set_dptx: 4, 3;

    /// Error Window
    ///
    /// Determines the rande of the error window. See Section 4.3.1 of the datasheet.
    pub u8, from into ErrorWindow, ergx, set_ergx: 2, 1;
}

#[derive(Clone, Copy, Debug, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub(crate) enum DerivativeOptions {
    #[default]
    None = 0b00,
    Basic = 0b01,
    Step = 0b10,
    Both = 0b11,
}

#[derive(Clone, Copy, Debug, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub(crate) enum ErrorWindow {
    #[default]
    Rpm0 = 0b00,
    Rpm50 = 0b01,
    Rpm100 = 0b10,
    Rpm200 = 0b11,
}
