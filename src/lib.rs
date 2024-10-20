#![cfg_attr(not(test), no_std)]

use core::future::Future;

use embedded_hal_async as hal;
use hal::i2c::I2c;

mod error;
use error::Error;

mod types;
use types::*;

#[derive(Clone, Copy, Debug)]
struct FanSetting {
    enable: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum FanControl {
    Direct(u8),
    Speed(u8),
}

#[derive(Clone, Copy, Debug)]
pub enum FanSelect {
    Fan(u8),
}

/// Fetch a register from the device which applies to all fans
macro_rules! fetch_register {
    ($func:ident, $register:expr, $return_type:ty) => {
        fn $func(&mut self) -> impl Future<Output = Result<$return_type, Error>> {
            async {
                let mut data = [0];
                self.write_register($register, &mut data).await?;
                data[0].try_into().map_err(|_| Error::RegisterTypeConversion)
            }
        }
    };
}

/// Fetch a register from the device which applies to a specific fan
macro_rules! fetch_fan_register {
    ($func:ident, $offset:expr, $return_type:ty) => {
        fn $func(&mut self, sel: FanSelect) -> impl Future<Output = Result<$return_type, Error>> {
            async move {
                self.valid_fan(sel)?;
                let base = match sel {
                    FanSelect::Fan(fan) => match fan {
                        1 => FAN1_BASE,
                        2 => FAN2_BASE,
                        3 => FAN3_BASE,
                        4 => FAN4_BASE,
                        5 => FAN5_BASE,
                        _ => return Err(Error::InvalidFan),
                    },
                };

                let reg: Registers = (base + $offset).try_into().map_err(|_| Error::InvalidRegister)?;

                let mut data = [0];
                self.write_register(reg, &mut data).await?;
                Ok(data[0])
            }
        }
    };
}

/// Dump all the info and registers from the EMC230x Device
pub async fn dump_info(dev: &mut impl Emc230x) -> Result<(), Error> {
    macro_rules! defmt_info_register {
        ($dev:expr, $reg:tt) => {
            let value = $dev.$reg().await?;
            defmt::info!("{}: {:#04x}", stringify!($reg), value);
        };
    }

    macro_rules! defmt_info_fan_register {
        ($dev:expr, $reg:tt, $fan:expr) => {
            let value = $dev.$reg(FanSelect::Fan($fan)).await?;
            defmt::info!("{}: {:#04x}", stringify!($reg), value);
        };
    }

    let count = dev.count();

    defmt::info!("Address: {:#04x}", dev.address());
    defmt::info!("Fan Count: {}", count);

    defmt_info_register!(dev, software_lock);
    defmt_info_register!(dev, product_features);
    defmt_info_register!(dev, product_id);

    defmt_info_register!(dev, config);
    defmt_info_register!(dev, status);
    defmt_info_register!(dev, stall_status);
    defmt_info_register!(dev, spin_status);
    defmt_info_register!(dev, drive_fail_status);
    defmt_info_register!(dev, interrupt_enable);
    defmt_info_register!(dev, pwm_polarity_config);
    defmt_info_register!(dev, pwm_output_config);
    defmt_info_register!(dev, pwm_base_f45);
    defmt_info_register!(dev, pwm_base_f123);

    for fan in 1..=count {
        defmt::info!("Fan: {} ----------------------", fan);
        defmt_info_fan_register!(dev, fan_setting, fan);
        defmt_info_fan_register!(dev, pwm_divide, fan);
        defmt_info_fan_register!(dev, fan_configuration1, fan);
        defmt_info_fan_register!(dev, fan_configuration2, fan);
        defmt_info_fan_register!(dev, gain, fan);
        defmt_info_fan_register!(dev, spin_up_configuration, fan);
        defmt_info_fan_register!(dev, max_step, fan);
        defmt_info_fan_register!(dev, minimum_drive, fan);
        defmt_info_fan_register!(dev, valid_tach_count, fan);
        defmt_info_fan_register!(dev, drive_fail_band_low_byte, fan);
        defmt_info_fan_register!(dev, drive_fail_band_high_byte, fan);
        defmt_info_fan_register!(dev, tach_target_low_byte, fan);
        defmt_info_fan_register!(dev, tach_target_high_byte, fan);
        defmt_info_fan_register!(dev, tach_reading_high_byte, fan);
        defmt_info_fan_register!(dev, tach_read_low_byte, fan);
    }

    Ok(())
}

pub trait Emc230x {
    fn mode(&mut self, sel: FanSelect) -> impl Future<Output = Result<FanControl, Error>> {
        async { todo!() }
    }

    fn set_mode(&mut self, sel: FanSelect, mode: FanControl) -> Result<(), Error> {
        self.valid_fan(sel)?;
        match mode {
            FanControl::Direct(_duty) => {
                todo!()
                // self.set_direct(duty).await
            }
            FanControl::Speed(_rpm) => {
                todo!()
                // self.set_speed(rpm).await
            }
        }
    }

    fn address(&self) -> u8;

    /// Write a value to a register on the device
    fn write_register(
        &mut self,
        reg: Registers,
        data: &mut [u8],
    ) -> impl Future<Output = Result<(), Error>>;

    /// Read a value from a register on the device
    fn read_register(&mut self, reg: Registers) -> impl Future<Output = Result<u8, Error>>;

    /// Get the number of fans the device supports
    fn count(&self) -> u8;

    /// Determine if the fan number is valid
    fn valid_fan(&self, select: FanSelect) -> Result<(), Error> {
        match select {
            FanSelect::Fan(fan) => {
                if fan <= self.count() && fan != 0 {
                    Ok(())
                } else {
                    Err(Error::InvalidFan)
                }
            }
        }
    }

    // General register access
    fetch_register!(config, Registers::Configuration, u8);
    fetch_register!(status, Registers::FanStatus, u8);
    fetch_register!(stall_status, Registers::FanStallStatus, u8);
    fetch_register!(spin_status, Registers::FanSpinStatus, u8);
    fetch_register!(drive_fail_status, Registers::DriveFailStatus, u8);
    fetch_register!(interrupt_enable, Registers::FanInterruptEnable, u8);
    fetch_register!(pwm_polarity_config, Registers::PwmPolarityConfig, u8);
    fetch_register!(pwm_output_config, Registers::PwmOutputConfig, u8);
    fetch_register!(pwm_base_f45, Registers::PwmBaseF45, u8);
    fetch_register!(pwm_base_f123, Registers::PwmBaseF123, u8);

    // Fan specific register access
    fetch_fan_register!(fan_setting, FAN_SETTING_OFFSET, u8);
    fetch_fan_register!(pwm_divide, PWM_DIVIDE_OFFSET, u8);
    fetch_fan_register!(fan_configuration1, FAN_CONFIGURATION1_OFFSET, u8);
    fetch_fan_register!(fan_configuration2, FAN_CONFIGURATION2_OFFSET, u8);
    fetch_fan_register!(gain, GAIN_OFFSET, u8);
    fetch_fan_register!(spin_up_configuration, FAN_SPIN_UP_CONFIGURATION_OFFSET, u8);
    fetch_fan_register!(max_step, FAN_MAX_STEP_OFFSET, u8);
    fetch_fan_register!(minimum_drive, FAN_MINIMUM_DRIVE_OFFSET, u8);
    fetch_fan_register!(valid_tach_count, FAN_VALID_TACH_COUNT_OFFSET, u8);
    fetch_fan_register!(drive_fail_band_low_byte, FAN_DRIVE_FAIL_BAND_LOW_BYTE_OFFSET, u8);
    fetch_fan_register!(drive_fail_band_high_byte, FAN_DRIVE_FAIL_BAND_HIGH_BYTE_OFFSET, u8);
    fetch_fan_register!(tach_target_low_byte, TACH_TARGET_LOW_BYTE_OFFSET, u8);
    fetch_fan_register!(tach_target_high_byte, TACH_TARGET_HIGH_BYTE_OFFSET, u8);
    fetch_fan_register!(tach_reading_high_byte, TACH_READING_HIGH_BYTE_OFFSET, u8);
    fetch_fan_register!(tach_read_low_byte, TACH_READ_LOW_BYTE_OFFSET, u8);

    // Chip registers
    fetch_register!(software_lock, Registers::SoftwareLock, u8);
    fetch_register!(product_features, Registers::ProductFeatures, u8);
    fetch_register!(product_id, Registers::ProductId, ProductId);
}

pub struct Emc2301<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C: I2c> Emc2301<I2C> {
    pub fn new(i2c: I2C, addr: u8) -> Self {
        Self { i2c, address: addr }
    }
}

impl<I2C: I2c> Emc230x for Emc2301<I2C> {
    fn count(&self) -> u8 {
        1
    }

    fn address(&self) -> u8 {
        self.address
    }

    fn write_register(
        &mut self,
        reg: Registers,
        data: &mut [u8],
    ) -> impl Future<Output = Result<(), Error>> {
        async {
            let addr = self.address();
            self.i2c
                .write_read(addr, &[reg as u8], data)
                .await
                .map_err(|_| Error::I2c)
        }
    }

    fn read_register(&mut self, _reg: Registers) -> impl Future<Output = Result<u8, Error>> {
        async { todo!() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
