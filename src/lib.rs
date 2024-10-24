#![cfg_attr(not(test), no_std)]

use core::future::Future;

use embedded_hal_async as hal;
use hal::i2c::I2c;

mod error;
use error::Error;

mod registers;
use registers::*;

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
        pub fn $func(&mut self) -> impl Future<Output = Result<$return_type, Error>> + '_ {
            async {
                Ok(self
                    .read_register($register)
                    .await?
                    .try_into()
                    .map_err(|_| Error::RegisterTypeConversion)?)
            }
        }
    };
}

/// Fetch a register from the device which applies to a specific fan
macro_rules! fetch_fan_register {
    ($func:ident, $offset:expr, $return_type:ty) => {
        pub fn $func(
            &mut self,
            sel: FanSelect,
        ) -> impl Future<Output = Result<$return_type, Error>> + '_ {
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

                let reg: Register = (base + $offset)
                    .try_into()
                    .map_err(|_| Error::InvalidRegister)?;

                Ok(self.read_register(reg).await?)
            }
        }
    };
}

/// Dump all the info and registers from the EMC230x Device
pub async fn dump_info<I2C: I2c>(dev: &mut Emc230x<I2C>) -> Result<(), Error> {
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

pub struct Emc230x<I2C> {
    /// I2C bus
    i2c: I2C,

    /// I2C address of the device
    address: u8,

    /// Number of fans the device supports
    count: u8,
}

impl<I2C: I2c> Emc230x<I2C> {
    /// Probe the I2C bus for an EMC230x device at the specified address
    pub fn probe(i2c: I2C, address: u8) -> impl Future<Output = Result<Self, Error>> {
        async move {
            let mut i2c = i2c;
            let id = &mut [0];

            // Manually setup the read to the ProductId register because the structure isn't formed yet
            i2c.write_read(address, &[Register::ProductId as u8], id)
                .await
                .map_err(|_| Error::I2c)?;

            let id: ProductId = id[0]
                .try_into()
                .map_err(|_| Error::RegisterTypeConversion)?;

            match id.into() {
                ProductId::Emc2301 => Ok(Self {
                    i2c,
                    address,
                    count: 1,
                }),
                ProductId::Emc2302 => Ok(Self {
                    i2c,
                    address,
                    count: 2,
                }),
                ProductId::Emc2303 => Ok(Self {
                    i2c,
                    address,
                    count: 3,
                }),
                ProductId::Emc2305 => Ok(Self {
                    i2c,
                    address,
                    count: 5,
                }),
            }
        }
    }

    /// Get the I2C address of the device
    fn address(&self) -> u8 {
        self.address
    }

    /// Get the number of fans the device supports
    fn count(&self) -> u8 {
        self.count
    }

    fn mode(&mut self, sel: FanSelect) -> impl Future<Output = Result<FanControl, Error>> {
        async { todo!() }
    }

    fn set_mode(&mut self, sel: FanSelect, mode: FanControl) -> Result<(), Error> {
        self.valid_fan(sel)?;
        match mode {
            FanControl::Direct(_duty) => {
                let set_value = _duty * (100 / 255);
                // let result = self.
                todo!()
                // self.set_direct(duty).await
            }
            FanControl::Speed(_rpm) => {
                todo!()
                // self.set_speed(rpm).await
            }
        }
    }

    /// Write a value to a register on the device
    fn write_register<'a>(
        &'a mut self,
        reg: Register,
        data: u8,
    ) -> impl Future<Output = Result<(), Error>> + 'a {
        async move {
            let addr = self.address();
            let data = [reg as u8, data];
            self.i2c.write(addr, &data).await.map_err(|_| Error::I2c)
        }
    }

    /// Read a value from a register on the device
    fn read_register<'a>(
        &'a mut self,
        reg: Register,
    ) -> impl Future<Output = Result<u8, Error>> + 'a {
        async {
            let addr = self.address();
            let mut data = [0];
            self.i2c
                .write_read(addr, &[reg as u8], data.as_mut_slice())
                .await
                .map_err(|_| Error::I2c)?;
            Ok(data[0])
        }
    }

    /// Determine if the fan number is valid by comparing it to the number of fans the device supports.
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
    fetch_register!(config, Register::Configuration, u8);
    fetch_register!(status, Register::FanStatus, u8);
    fetch_register!(stall_status, Register::FanStallStatus, u8);
    fetch_register!(spin_status, Register::FanSpinStatus, u8);
    fetch_register!(drive_fail_status, Register::DriveFailStatus, u8);
    fetch_register!(interrupt_enable, Register::FanInterruptEnable, u8);
    fetch_register!(pwm_polarity_config, Register::PwmPolarityConfig, u8);
    fetch_register!(pwm_output_config, Register::PwmOutputConfig, u8);
    fetch_register!(pwm_base_f45, Register::PwmBaseF45, u8);
    fetch_register!(pwm_base_f123, Register::PwmBaseF123, u8);

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
    fetch_fan_register!(
        drive_fail_band_low_byte,
        FAN_DRIVE_FAIL_BAND_LOW_BYTE_OFFSET,
        u8
    );
    fetch_fan_register!(
        drive_fail_band_high_byte,
        FAN_DRIVE_FAIL_BAND_HIGH_BYTE_OFFSET,
        u8
    );
    fetch_fan_register!(tach_target_low_byte, TACH_TARGET_LOW_BYTE_OFFSET, u8);
    fetch_fan_register!(tach_target_high_byte, TACH_TARGET_HIGH_BYTE_OFFSET, u8);
    fetch_fan_register!(tach_reading_high_byte, TACH_READING_HIGH_BYTE_OFFSET, u8);
    fetch_fan_register!(tach_read_low_byte, TACH_READ_LOW_BYTE_OFFSET, u8);

    // Chip registers
    fetch_register!(software_lock, Register::SoftwareLock, u8);
    fetch_register!(product_features, Register::ProductFeatures, u8);
    fetch_register!(product_id, Register::ProductId, ProductId);
}

#[cfg(test)]
mod tests {
    use super::*;
}
