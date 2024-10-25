#![cfg_attr(not(test), no_std)]

use core::future::Future;

use embedded_hal_async as hal;
use hal::i2c::I2c;

mod error;
use error::Error;

mod registers;
use registers::*;

#[derive(Clone, Copy, Debug)]
pub enum FanControl {
    Direct(u8),
    Speed(u16),
}

#[derive(Clone, Copy, Debug)]
pub enum FanSelect {
    Fan(u8),
}

/// Fetch a register from the device which applies to all fans
macro_rules! fetch_register {
    ($func:ident, $register:expr, $return_type:ty) => {
        pub async fn $func(&mut self) -> Result<$return_type, Error> {
            self.read_register($register)
                .await?
                .try_into()
                .map_err(|_| Error::RegisterTypeConversion)
        }
    };
}

/// Fetch a register from the device which applies to a specific fan
macro_rules! fan_register {
    ($get:ident, $set:ident, $offset:expr, $reg_type:ty) => {
        pub async fn $get(&mut self, sel: FanSelect) -> Result<$reg_type, Error> {
            self.valid_fan(sel)?;
            let reg = fan_register_address(sel, $offset)?;
            self.read_register(reg).await
        }

        pub async fn $set(&mut self, sel: FanSelect, value: $reg_type) -> Result<(), Error> {
            self.valid_fan(sel)?;
            let reg = fan_register_address(sel, $offset)?;
            self.write_register(reg, value).await?;
            Ok(())
        }
    };
}

/// Manually hack rounding the value because `core` doesn't have `round`
///
/// This is a terrible practice. Is there a better way to do this?
fn hacky_round(value: f64) -> u8 {
    // Interpret the value as a u8 first to get an integer value
    let raw = value as u8;

    // Reinterpret the integer value as a f64 and compare it to the original value
    if value - raw as f64 >= 0.5 {
        raw + 1
    } else {
        raw
    }
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
        defmt_info_fan_register!(dev, tach_reading_low_byte, fan);
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
    pub async fn probe(i2c: I2C, address: u8) -> Result<Self, Error> {
        let mut i2c = i2c;
        let id = &mut [0];

        // Manually setup the read to the ProductId register because the structure isn't formed yet
        i2c.write_read(address, &[Register::ProductId as u8], id)
            .await
            .map_err(|_| Error::I2c)?;

        let id: ProductId = id[0]
            .try_into()
            .map_err(|_| Error::RegisterTypeConversion)?;

        match id {
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

    /// Set the mode of the fan
    pub async fn set_mode(&mut self, sel: FanSelect, mode: FanControl) -> Result<(), Error> {
        self.valid_fan(sel)?;
        match mode {
            FanControl::Direct(duty) => {
                self.set_duty_cycle(sel, duty).await?;
            }
            FanControl::Speed(rpm) => {
                self.set_rpm(sel, rpm).await?;

                let mut config = self.fan_configuration1(sel).await?;
                config |= 0x80;
                self.set_fan_configuration1(sel, config).await?;
            }
        }

        Ok(())
    }

    /// Fetch the current duty cycle of the fan
    pub async fn duty_cycle(&mut self, sel: FanSelect) -> Result<u8, Error> {
        self.valid_fan(sel)?;
        let raw = self.fan_setting(sel).await?;
        let duty = (raw as f64 / 255.0) * 100.0;
        let duty = hacky_round(duty);
        Ok(duty)
    }

    /// Set the duty cycle of the fan
    pub async fn set_duty_cycle(&mut self, sel: FanSelect, duty: u8) -> Result<(), Error> {
        let raw = (duty as f64 / 100.0) * 255.0;
        let raw = hacky_round(raw);

        defmt::warn!("Setting fan to {}% (register: {:#04x})", duty, raw);
        self.set_fan_setting(sel, raw).await?;
        Ok(())
    }

    /// Fetch the current RPM of the fan
    pub async fn rpm(&mut self, sel: FanSelect) -> Result<u64, Error> {
        self.valid_fan(sel)?;
        let raw_low = self.tach_reading_low_byte(sel).await?;
        let raw_high = self.tach_reading_high_byte(sel).await?;

        let raw = u16::from_le_bytes([raw_low, raw_high]) >> 3;
        let rpm = self.calc_raw_rpm(raw);

        Ok(rpm as u64)
    }

    /// Set the target RPM of the fan
    pub async fn set_rpm(&mut self, sel: FanSelect, rpm: u16) -> Result<(), Error> {
        self.valid_fan(sel)?;

        let raw = self.calc_raw_rpm(rpm);
        let count = (raw << 3).to_le_bytes();

        self.set_tach_target_low_byte(sel, count[0]).await?;
        self.set_tach_target_high_byte(sel, count[1]).await?;
        defmt::warn!(
            "Setting RPM to {} (register: {:#04x}, {:#04x})",
            rpm,
            raw,
            count
        );
        Ok(())
    }

    /// Calculate either the RPM or raw value of the RPM based on the input value.
    fn calc_raw_rpm(&self, value: u16) -> u16 {
        let poles = 2.0;
        let n = 5.0;
        let m = 1.0;
        let f_tach = 32768.0;
        let _simplified = 3_932_160.0;

        (((1.0 / poles) * (n - 1.0)) / (value as f64 * (1.0 / m)) * f_tach * 60.0) as u16
    }

    /// Write a value to a register on the device
    async fn write_register(&mut self, reg: Register, data: u8) -> Result<(), Error> {
        let addr = self.address();
        let data = [reg as u8, data];
        self.i2c.write(addr, &data).await.map_err(|_| Error::I2c)
    }

    /// Read a value from a register on the device
    async fn read_register(&mut self, reg: Register) -> Result<u8, Error> {
        let addr = self.address();
        let mut data = [0];
        self.i2c
            .write_read(addr, &[reg as u8], data.as_mut_slice())
            .await
            .map_err(|_| Error::I2c)?;
        Ok(data[0])
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
    fan_register!(fan_setting, set_fan_setting, FAN_SETTING_OFFSET, u8);
    fan_register!(pwm_divide, set_pwm_divide, PWM_DIVIDE_OFFSET, u8);
    fan_register!(
        fan_configuration1,
        set_fan_configuration1,
        FAN_CONFIGURATION1_OFFSET,
        u8
    );
    fan_register!(
        fan_configuration2,
        set_fan_configuration2,
        FAN_CONFIGURATION2_OFFSET,
        u8
    );
    fan_register!(gain, set_gain, GAIN_OFFSET, u8);
    fan_register!(
        spin_up_configuration,
        set_spin_up_configuration,
        FAN_SPIN_UP_CONFIGURATION_OFFSET,
        u8
    );
    fan_register!(max_step, set_max_step, FAN_MAX_STEP_OFFSET, u8);
    fan_register!(
        minimum_drive,
        set_minimum_drive,
        FAN_MINIMUM_DRIVE_OFFSET,
        u8
    );
    fan_register!(
        valid_tach_count,
        set_valid_tach_count,
        FAN_VALID_TACH_COUNT_OFFSET,
        u8
    );
    fan_register!(
        drive_fail_band_low_byte,
        set_drive_fail_band_low_byte,
        FAN_DRIVE_FAIL_BAND_LOW_BYTE_OFFSET,
        u8
    );
    fan_register!(
        drive_fail_band_high_byte,
        set_drive_fail_band_high_byte,
        FAN_DRIVE_FAIL_BAND_HIGH_BYTE_OFFSET,
        u8
    );
    fan_register!(
        tach_target_low_byte,
        set_tach_target_low_byte,
        TACH_TARGET_LOW_BYTE_OFFSET,
        u8
    );
    fan_register!(
        tach_target_high_byte,
        set_tach_target_high_byte,
        TACH_TARGET_HIGH_BYTE_OFFSET,
        u8
    );
    fan_register!(
        tach_reading_high_byte,
        set_tach_reading_high_byte,
        TACH_READING_HIGH_BYTE_OFFSET,
        u8
    );
    fan_register!(
        tach_reading_low_byte,
        set_tach_reading_low_byte,
        TACH_READ_LOW_BYTE_OFFSET,
        u8
    );

    // Chip registers
    fetch_register!(software_lock, Register::SoftwareLock, u8);
    fetch_register!(product_features, Register::ProductFeatures, u8);
    fetch_register!(product_id, Register::ProductId, ProductId);
}

#[cfg(test)]
mod tests {
    use super::*;
}
