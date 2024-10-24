#[derive(Debug)]
pub enum Error {
    /// I2C bus error
    I2c,

    /// Device return invalid device identifier
    InvalidDeviceId,

    /// Invalid fan number
    InvalidFan,

    /// Invalid register
    InvalidRegister,

    /// Failed to convert a raw register value into a specific type
    RegisterTypeConversion,

    /// Selected fan speed out of range
    SpeedOutOfRange,
}

impl defmt::Format for Error {
    fn format(&self, f: defmt::Formatter) {
        match self {
            Error::I2c => defmt::write!(f, "I2c"),
            Error::InvalidDeviceId => defmt::write!(f, "InvalidDeviceId"),
            Error::InvalidFan => defmt::write!(f, "InvalidFan"),
            Error::InvalidRegister => defmt::write!(f, "InvalidRegister"),
            Error::RegisterTypeConversion => defmt::write!(f, "RegisterTypeConversion"),
            Error::SpeedOutOfRange => defmt::write!(f, "SpeedOutOfRange"),
        }
    }
}
