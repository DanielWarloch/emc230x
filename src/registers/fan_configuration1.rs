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
    #[register(offset = 0x02, default = 0x2B)]
    pub struct FanConfiguration1(u8);
    impl Debug;

    /// Closed loop algorithm control
    ///
    /// 1: Closed loop algorithm is enabled. Changes to Fan Setting register are ignored.
    ///
    /// 0: Closed loop algorithm is disabled. The device is placed in Direct Setting mode.
    ///    Changes to the Fan Setting register will change the PWM duty cycle.
    pub enagx, set_enagx: 7;

    /// RPM Range
    ///
    /// The minimum fan speed measured and reported by the device.
    pub u8, from into Range, rngx, set_rngx: 6, 5;

    /// The number of edges to sample when calculating RPM.
    pub u8, from into Edges, edgx, set_edgx: 4, 3;

    /// Update Time.
    ///
    /// The PID update rate for closed loop control. The Update Time, along with
    /// the Fan Step register, is used to control the ramp rate of the drive response
    /// to provide a cleaner transition of the actual fan operation as the desired
    /// fan speed changes.
    pub u8, from into UpdateTime, udtx, set_udtx: 2, 0;
}

#[derive(Clone, Copy, Debug, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Range {
    Rpm500 = 0b00,
    #[default]
    Rpm1000 = 0b01,
    Rpm2000 = 0b10,
    Rpm4000 = 0b11,
}

impl Range {
    pub fn min_rpm(&self) -> u16 {
        match self {
            Range::Rpm500 => 500,
            Range::Rpm1000 => 1000,
            Range::Rpm2000 => 2000,
            Range::Rpm4000 => 4000,
        }
    }

    pub fn tach_count_multiplier(&self) -> u8 {
        match self {
            Range::Rpm500 => 1,
            Range::Rpm1000 => 2,
            Range::Rpm2000 => 4,
            Range::Rpm4000 => 8,
        }
    }
}

#[derive(Clone, Copy, Debug, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Edges {
    Sample3 = 0b00,
    #[default]
    Sample5 = 0b01,
    Sample7 = 0b10,
    Sample9 = 0b11,
}

impl Edges {
    pub fn num_edges(&self) -> u8 {
        match self {
            Edges::Sample3 => 3,
            Edges::Sample5 => 5,
            Edges::Sample7 => 7,
            Edges::Sample9 => 9,
        }
    }

    pub fn poles(&self) -> u8 {
        match self {
            Edges::Sample9 => 4,
            Edges::Sample7 => 3,
            Edges::Sample5 => 2,
            Edges::Sample3 => 1,
        }
    }

    pub fn tach_multiplier(&self) -> f64 {
        match self {
            Edges::Sample9 => 2.0,
            Edges::Sample7 => 1.5,
            Edges::Sample5 => 1.0,
            Edges::Sample3 => 0.5,
        }
    }
}

#[derive(Clone, Copy, Debug, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum UpdateTime {
    UpdateTime100ms = 0b000,
    UpdateTime200ms = 0b001,
    UpdateTime300ms = 0b010,
    #[default]
    UpdateTime400ms = 0b011,
    UpdateTime500ms = 0b100,
    UpdateTime800ms = 0b101,
    UpdateTime1200ms = 0b110,
    UpdateTime1600ms = 0b111,
}

impl UpdateTime {
    pub fn millis(&self) -> u16 {
        match self {
            UpdateTime::UpdateTime100ms => 100,
            UpdateTime::UpdateTime200ms => 200,
            UpdateTime::UpdateTime300ms => 300,
            UpdateTime::UpdateTime400ms => 400,
            UpdateTime::UpdateTime500ms => 500,
            UpdateTime::UpdateTime800ms => 800,
            UpdateTime::UpdateTime1200ms => 1200,
            UpdateTime::UpdateTime1600ms => 1600,
        }
    }
}
