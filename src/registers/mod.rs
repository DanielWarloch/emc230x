use crate::{Error, FanSelect};
pub(crate) use configuration::Configuration;
pub(crate) use drive_fail_band::{DriveFailBandHigh, DriveFailBandLow};
pub(crate) use fan_configuration1::FanConfiguration1;
pub(crate) use fan_configuration2::FanConfiguration2;
pub(crate) use fan_drive_fail_status::FanDriveFailStatus;
pub(crate) use fan_drive_setting::FanDriveSetting;
pub(crate) use fan_interrupt_enable::FanInterruptEnable;
pub(crate) use fan_min_drive::FanMinimumDrive;
pub(crate) use fan_spin_status::FanSpinStatus;
pub(crate) use fan_spin_up_config::FanSpinUpConfig;
pub(crate) use fan_stall_status::FanStallStatus;
pub(crate) use fan_status::FanStatus;
pub(crate) use max_step_size::MaxStepSize;
pub(crate) use pid_gain::PidGain;
pub(crate) use product_features::ProductFeatures;
pub(crate) use product_id::ProductId;
pub(crate) use pwm_base::{PwmBase123, PwmBase45};
pub(crate) use pwm_divide::PwmDivide;
pub(crate) use pwm_output_config::PwmOutputConfig;
pub(crate) use pwm_polarity_config::PwmPolarityConfig;
pub(crate) use software_lock::SoftwareLock;
pub(crate) use tach_reading::{TachReadingHigh, TachReadingLow};
pub(crate) use tach_target::{TachTargetHigh, TachTargetLow};
pub(crate) use valid_tach_count::ValidTachCount;

pub(crate) mod configuration;
pub(crate) mod drive_fail_band;
pub(crate) mod fan_configuration1;
pub(crate) mod fan_configuration2;
pub(crate) mod fan_drive_fail_status;
pub(crate) mod fan_drive_setting;
pub(crate) mod fan_interrupt_enable;
pub(crate) mod fan_min_drive;
pub(crate) mod fan_spin_status;
pub(crate) mod fan_spin_up_config;
pub(crate) mod fan_stall_status;
pub(crate) mod fan_status;
pub(crate) mod max_step_size;
pub(crate) mod pid_gain;
pub(crate) mod product_features;
pub(crate) mod product_id;
pub(crate) mod pwm_base;
pub(crate) mod pwm_divide;
pub(crate) mod pwm_output_config;
pub(crate) mod pwm_polarity_config;
pub(crate) mod software_lock;
pub(crate) mod tach_reading;
pub(crate) mod tach_target;
pub(crate) mod valid_tach_count;

pub(crate) const FAN1_BASE: u8 = 0x30;
pub(crate) const FAN2_BASE: u8 = 0x40;
pub(crate) const FAN3_BASE: u8 = 0x50;
pub(crate) const FAN4_BASE: u8 = 0x60;
pub(crate) const FAN5_BASE: u8 = 0x70;

pub(crate) fn fan_register_address(sel: FanSelect, offset: u8) -> Result<u8, Error> {
    let base = match sel.0 {
        1 => FAN1_BASE,
        2 => FAN2_BASE,
        3 => FAN3_BASE,
        4 => FAN4_BASE,
        5 => FAN5_BASE,
        _ => return Err(Error::InvalidFan),
    };

    Ok(base + offset)
}

#[derive(Clone, Copy, Debug, emc230x_macros::RegisterAddress)]
#[register(address = 0xFE, default = 0x5D)]
pub struct ManufacturerId(u8);

impl ManufacturerId {
    pub fn mfg_id(&self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, emc230x_macros::RegisterAddress)]
#[register(address = 0xFF, default = 0x80)]
pub struct SiliconRevision(u8);

pub(crate) trait RegisterAddress {
    const ADDRESS: u8;
}

pub(crate) trait RegisterOffset {
    const OFFSET: u8;

    const FAN1_ADDRESS: u8 = FAN1_BASE + Self::OFFSET;
    const FAN2_ADDRESS: u8 = FAN2_BASE + Self::OFFSET;
    const FAN3_ADDRESS: u8 = FAN3_BASE + Self::OFFSET;
    const FAN4_ADDRESS: u8 = FAN4_BASE + Self::OFFSET;
    const FAN5_ADDRESS: u8 = FAN5_BASE + Self::OFFSET;
}
