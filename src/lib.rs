#![cfg_attr(not(test), no_std)]

use core::future::Future;
use embedded_hal_async as hal;
use hal::i2c::I2c;

use error::Error;
use registers::*;

mod error;
mod registers;

#[derive(Clone, Copy, Debug)]
pub enum FanControl {
    DutyCycle(u8),
    Rpm(u16),
}

#[derive(Clone, Copy, Debug)]
pub enum FanSelect {
    Fan(u8),
}

macro_rules! register_ro {
    ($get:ident, $return_type:ty) => {
        pub async fn $get(&mut self) -> Result<$return_type, Error> {
            self.read_register(<$return_type>::ADDRESS)
                .await?
                .try_into()
                .map_err(|_| Error::RegisterTypeConversion)
        }
    };
}

/// Fetch a register from the device which applies to all fans
macro_rules! register {
    ($get:ident, $set:ident, $return_type:ty) => {
        pub async fn $get(&mut self) -> Result<$return_type, Error> {
            self.read_register(<$return_type>::ADDRESS)
                .await?
                .try_into()
                .map_err(|_| Error::RegisterTypeConversion)
        }

        pub async fn $set(&mut self, value: $return_type) -> Result<(), Error> {
            self.write_register(<$return_type>::ADDRESS, value.into())
                .await?;
            Ok(())
        }
    };
}

/// Fetch a register from the device which applies to a specific fan
macro_rules! fan_register {
    ($get:ident, $set:ident, $reg_type:ty) => {
        pub async fn $get(&mut self, sel: FanSelect) -> Result<$reg_type, Error> {
            self.valid_fan(sel)?;
            let reg = fan_register_address(sel, <$reg_type>::OFFSET)?;
            let value = self.read_register(reg).await?;
            Ok(value.into())
        }

        pub async fn $set(&mut self, sel: FanSelect, value: $reg_type) -> Result<(), Error> {
            self.valid_fan(sel)?;
            let reg = fan_register_address(sel, <$reg_type>::OFFSET)?;
            self.write_register(reg, value.into()).await?;
            Ok(())
        }
    };
}

/// Manually hack rounding the value because `core` doesn't have `round`
///
/// This is a terrible practice. Is there a better way to do this?
pub(crate) fn hacky_round(value: f64) -> u8 {
    // Interpret the value as a u8 first to get an integer value
    let raw = value as u8;

    // Reinterpret the integer value as a f64 and compare it to the original value
    if value - raw as f64 >= 0.5 {
        raw + 1
    } else {
        raw
    }
}

pub struct Emc230x<I2C> {
    /// I2C bus
    i2c: I2C,

    /// I2C address of the device
    address: u8,

    /// Device Product Identifier
    pid: ProductId,

    /// Configurable number of poles in a fan. Typically 2.
    poles: [u8; 5],
}

impl<I2C: I2c> Emc230x<I2C> {
    const TACH_FREQUENCY_HZ: f64 = 32_768.0;
    const _SIMPLIFIED_RPM_FACTOR: f64 = 3_932_160.0;

    /// Probe the I2C bus for an EMC230x device at the specified address
    pub async fn probe(i2c: I2C, address: u8) -> Result<Self, Error> {
        let mut i2c = i2c;
        let pid = &mut [0];

        // Manually setup the read to the ProductId register because the structure isn't formed yet
        i2c.write_read(address, &[ProductId::ADDRESS], pid)
            .await
            .map_err(|_| Error::I2c)?;

        let pid: ProductId = pid[0]
            .try_into()
            .map_err(|_| Error::RegisterTypeConversion)?;

        // Assume 2 poles for all fans by default
        let poles = [2, 2, 2, 2, 2];

        // Form the device so that some defaults can be set
        let mut dev = Self {
            i2c,
            address,
            pid,
            poles,
        };

        // Set all fan outputs to push-pull to avoid waveform distortion
        let mut output_cfg = pwm_output_config::PwmOutputConfig::default();
        let count = dev.count();
        for fan in 1..=count {
            output_cfg.push_pull(fan);
        }
        dev.set_pwm_output_config(output_cfg).await?;

        // Set RPM range to 500 RPM for all drives to capture slower fans
        for fan in 1..=count {
            let mut cfg = dev.fan_configuration1(FanSelect::Fan(fan)).await?;
            cfg.set_rngx(fan_configuration1::Range::Rpm500);
            dev.set_fan_configuration1(FanSelect::Fan(fan), cfg).await?;
        }

        // Device is configured
        Ok(dev)
    }

    /// Get the I2C address of the device
    fn address(&self) -> u8 {
        self.address
    }

    /// Get the number of fans the device supports
    fn count(&self) -> u8 {
        self.pid.num_fans()
    }

    /// Get the number of poles for the selected fan (used in RPM calculations)
    pub fn fan_poles(&self, sel: FanSelect) -> Result<u8, Error> {
        self.valid_fan(sel)?;
        match sel {
            FanSelect::Fan(fan) => Ok(self.poles[fan as usize]),
        }
    }

    /// Set the number of poles for the selected fan (used in RPM calculations)
    ///
    /// It is unlikely that this value will need to change unless a non-standard fan is used.
    /// If it does need to change, there are likely other configuration changes that need to
    /// happen as well.
    pub fn set_fan_poles(&mut self, sel: FanSelect, poles: u8) -> Result<(), Error> {
        self.valid_fan(sel)?;
        match sel {
            FanSelect::Fan(fan) => {
                self.poles[fan as usize] = poles;
            }
        }

        Ok(())
    }

    /// Get the tachometer frequency of the device
    fn tach_freq(&self) -> f64 {
        Self::TACH_FREQUENCY_HZ
    }

    fn _mode(&mut self, _sel: FanSelect) -> impl Future<Output = Result<FanControl, Error>> {
        async { todo!() }
    }

    /// Set the mode of the fan
    pub async fn set_mode(&mut self, sel: FanSelect, mode: FanControl) -> Result<(), Error> {
        self.valid_fan(sel)?;
        match mode {
            FanControl::DutyCycle(duty) => {
                self.set_duty_cycle(sel, duty).await?;
            }
            FanControl::Rpm(rpm) => {
                self.set_rpm(sel, rpm).await?;

                let mut config = self.fan_configuration1(sel).await?;
                config.set_enagx(true);
                self.set_fan_configuration1(sel, config).await?;
            }
        }

        Ok(())
    }

    /// Fetch the current duty cycle of the fan
    pub async fn duty_cycle(&mut self, sel: FanSelect) -> Result<u8, Error> {
        self.valid_fan(sel)?;
        let drive = self.fan_setting(sel).await?;
        let duty = drive.duty_cycle();
        Ok(duty)
    }

    /// Set the duty cycle of the fan
    pub async fn set_duty_cycle(&mut self, sel: FanSelect, duty: u8) -> Result<(), Error> {
        self.valid_fan(sel)?;
        let drive = FanDriveSetting::from_duty_cycle(duty);
        self.set_fan_setting(sel, drive).await?;
        Ok(())
    }

    /// Fetch the current RPM of the fan
    pub async fn rpm(&mut self, sel: FanSelect) -> Result<u64, Error> {
        self.valid_fan(sel)?;
        let raw_low = self.tach_reading_low_byte(sel).await?;
        let raw_high = self.tach_reading_high_byte(sel).await?;

        let raw = u16::from_le_bytes([raw_low.into(), raw_high.into()]) >> 3;
        let rpm = self.calc_raw_rpm(sel, raw).await?;

        Ok(rpm as u64)
    }

    /// Set the target RPM of the fan
    pub async fn set_rpm(&mut self, sel: FanSelect, rpm: u16) -> Result<(), Error> {
        self.valid_fan(sel)?;

        let raw = self.calc_raw_rpm(sel, rpm).await?;
        let count = (raw << 3).to_le_bytes();

        self.set_tach_target_low_byte(sel, count[0].into()).await?;
        self.set_tach_target_high_byte(sel, count[1].into()).await?;
        Ok(())
    }

    /// Minimum configured duty cycle the fan will run at.
    pub async fn min_duty(&mut self, sel: FanSelect) -> Result<u8, Error> {
        self.valid_fan(sel)?;
        let drive = self.minimum_drive(sel).await?;
        Ok(drive.duty_cycle())
    }

    /// Set the minimum duty cycle the fan will run at.
    pub async fn set_min_duty(&mut self, sel: FanSelect, duty: u8) -> Result<(), Error> {
        self.valid_fan(sel)?;
        let drive = FanMinimumDrive::from_duty_cycle(duty);
        self.set_minimum_drive(sel, drive).await?;
        Ok(())
    }

    /// Calculate either the RPM or raw value of the RPM based on the input value.
    async fn calc_raw_rpm(&mut self, sel: FanSelect, value: u16) -> Result<u16, Error> {
        let cfg = self.fan_configuration1(sel).await?;

        let poles = self.fan_poles(sel)? as f64;
        let n = cfg.edgx().num_edges() as f64;
        let m = cfg.rngx().tach_count_multiplier() as f64;
        let f_tach = self.tach_freq();

        Ok((((1.0 / poles) * (n - 1.0)) / (value as f64 * (1.0 / m)) * f_tach * 60.0) as u16)
    }

    /// Write a value to a register on the device
    async fn write_register(&mut self, reg: u8, data: u8) -> Result<(), Error> {
        let addr = self.address();
        let data = [reg, data];
        self.i2c.write(addr, &data).await.map_err(|_| Error::I2c)
    }

    /// Read a value from a register on the device
    async fn read_register(&mut self, reg: u8) -> Result<u8, Error> {
        let addr = self.address();
        let mut data = [0];
        self.i2c
            .write_read(addr, &[reg], data.as_mut_slice())
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
    register!(config, set_config, Configuration);
    register_ro!(status, FanStatus);
    register_ro!(stall_status, FanStallStatus);
    register_ro!(spin_status, FanSpinStatus);
    register_ro!(drive_fail_status, FanDriveFailStatus);
    register!(interrupt_enable, set_interrupt_enable, FanInterruptEnable);
    register!(pwm_polarity_config, set_pwm_polarity_config, PwmPolarityConfig);
    register!(pwm_output_config, set_pwm_output_config, PwmOutputConfig);
    register!(pwm_base_f45, set_pwm_base_f45, PwmBase45);
    register!(pwm_base_f123, set_pwm_base_f123, PwmBase123);

    // Fan specific register access
    fan_register!(fan_setting, set_fan_setting, FanDriveSetting);
    fan_register!(pwm_divide, set_pwm_divide, PwmDivide);
    fan_register!(fan_configuration1, set_fan_configuration1, FanConfiguration1);
    fan_register!(fan_configuration2, set_fan_configuration2, FanConfiguration2);
    fan_register!(gain, set_gain, PidGain);
    fan_register!(spin_up_configuration, set_spin_up_configuration, FanSpinUpConfig);
    fan_register!(max_step, set_max_step, MaxStepSize);
    fan_register!(minimum_drive, set_minimum_drive, FanMinimumDrive);
    fan_register!(valid_tach_count, set_valid_tach_count, ValidTachCount);
    fan_register!(drive_fail_band_low_byte, set_drive_fail_band_low_byte, DriveFailBandLow);
    fan_register!(drive_fail_band_high_byte, set_drive_fail_band_high_byte, DriveFailBandHigh);
    fan_register!(tach_target_low_byte, set_tach_target_low_byte, TachTargetLow);
    fan_register!(tach_target_high_byte, set_tach_target_high_byte, TachTargetHigh);
    fan_register!(tach_reading_high_byte, set_tach_reading_high_byte, TachReadingHigh);
    fan_register!(tach_reading_low_byte, set_tach_reading_low_byte, TachReadingLow);

    // Chip registers
    register_ro!(software_lock, SoftwareLock);
    register_ro!(product_features, ProductFeatures);
    register_ro!(product_id, ProductId);

    /// Dump all the info and registers from the EMC230x Device
    pub async fn dump_info(&mut self) -> Result<(), Error> {
        macro_rules! defmt_info_register {
            ($dev:expr, $reg:tt) => {
                let value = $dev.$reg().await?;
                defmt::info!("{}: {:#04x}", stringify!($reg), u8::from(value));
            };
        }

        macro_rules! defmt_info_fan_register {
            ($dev:expr, $reg:tt, $fan:expr) => {
                let value = $dev.$reg(FanSelect::Fan($fan)).await?;
                defmt::info!("{}: {:#04x}", stringify!($reg), u8::from(value));
            };
        }

        let count = self.count();

        defmt::info!("Address: {:#04x}", self.address());
        defmt::info!("Fan Count: {}", count);

        defmt_info_register!(self, software_lock);
        defmt_info_register!(self, product_features);
        defmt_info_register!(self, product_id);

        defmt_info_register!(self, config);
        defmt_info_register!(self, status);
        defmt_info_register!(self, stall_status);
        defmt_info_register!(self, spin_status);
        defmt_info_register!(self, drive_fail_status);
        defmt_info_register!(self, interrupt_enable);
        defmt_info_register!(self, pwm_polarity_config);
        defmt_info_register!(self, pwm_output_config);
        defmt_info_register!(self, pwm_base_f45);
        defmt_info_register!(self, pwm_base_f123);

        for fan in 1..=count {
            defmt::info!("Fan: {} ----------------------", fan);
            defmt_info_fan_register!(self, fan_setting, fan);
            defmt_info_fan_register!(self, pwm_divide, fan);
            defmt_info_fan_register!(self, fan_configuration1, fan);
            defmt_info_fan_register!(self, fan_configuration2, fan);
            defmt_info_fan_register!(self, gain, fan);
            defmt_info_fan_register!(self, spin_up_configuration, fan);
            defmt_info_fan_register!(self, max_step, fan);
            defmt_info_fan_register!(self, minimum_drive, fan);
            defmt_info_fan_register!(self, valid_tach_count, fan);
            defmt_info_fan_register!(self, drive_fail_band_low_byte, fan);
            defmt_info_fan_register!(self, drive_fail_band_high_byte, fan);
            defmt_info_fan_register!(self, tach_target_low_byte, fan);
            defmt_info_fan_register!(self, tach_target_high_byte, fan);
            defmt_info_fan_register!(self, tach_reading_high_byte, fan);
            defmt_info_fan_register!(self, tach_reading_low_byte, fan);
        }

        Ok(())
    }
}
