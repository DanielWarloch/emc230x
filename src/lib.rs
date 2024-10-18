#![cfg_attr(not(test), no_std)]

use core::future::Future;

use embedded_hal_async as hal;
use hal::i2c::I2c;

mod error;
use error::Error;

mod types;
use types::Registers;

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
        value: &[u8],
    ) -> impl Future<Output = Result<(), Error>>;

    /// Read a value from a register on the device
    fn read_register(&mut self, reg: Registers) -> impl Future<Output = Result<u8, Error>>;

    fn write_read_register(
        &mut self,
        reg: Registers,
        data: &mut [u8],
    ) -> impl Future<Output = Result<(), Error>>;

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

    fn config(&mut self) -> impl Future<Output = Result<u8, Error>> {
        async {
            let mut data = [0];
            self.write_read_register(Registers::Configuration, &mut data)
                .await?;
            Ok(data[0])
        }
    }

    fn status(&mut self) -> impl Future<Output = Result<u8, Error>> {
        async {
            let mut data = [0];
            self.write_read_register(Registers::FanStatus, &mut data)
                .await?;
            Ok(data[0])
        }
    }

    fn stall_status(&mut self) -> impl Future<Output = Result<u8, Error>> {
        async {
            let mut data = [0];
            self.write_read_register(Registers::FanStallStatus, &mut data)
                .await?;
            Ok(data[0])
        }
    }

    fn spin_status(&mut self) -> impl Future<Output = Result<u8, Error>> {
        async {
            let mut data = [0];
            self.write_read_register(Registers::FanSpinStatus, &mut data)
                .await?;
            Ok(data[0])
        }
    }

    /// Value of registers that determine the final drive frequency of the PWM.
    fn pwm_divide(&mut self, sel: FanSelect) -> impl Future<Output = Result<u8, Error>> {
        async move {
            self.valid_fan(sel)?;
            let reg = match sel {
                FanSelect::Fan(fan) => match fan {
                    1 => Registers::Pwm1Divide,
                    2 => Registers::Pwm2Divide,
                    3 => Registers::Pwm3Divide,
                    _ => return Err(Error::InvalidFan),
                },
            };

            let mut data = [0];
            self.write_read_register(reg, &mut data).await?;
            Ok(data[0])
        }
    }
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
        _reg: Registers,
        _value: &[u8],
    ) -> impl Future<Output = Result<(), Error>> {
        async { todo!() }
    }

    fn read_register(&mut self, _reg: Registers) -> impl Future<Output = Result<u8, Error>> {
        async { todo!() }
    }

    fn write_read_register(
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
}

#[cfg(test)]
mod tests {
    use super::*;
}
