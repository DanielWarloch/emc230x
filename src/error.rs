#[derive(Debug)]
pub enum Error {
    /// I2C bus error
    I2c,

    /// Invalid fan number
    InvalidFan,

    /// Invalid register
    InvalidRegister,

    /// Failed to convert a raw register value into a specific type
    RegisterTypeConversion,
}

impl defmt::Format for Error {
    fn format(&self, f: defmt::Formatter) {
        match self {
            Error::I2c => defmt::write!(f, "I2c"),
            Error::InvalidFan => defmt::write!(f, "InvalidFan"),
            Error::InvalidRegister => defmt::write!(f, "InvalidRegister"),
            Error::RegisterTypeConversion => defmt::write!(f, "RegisterTypeConversion"),
        }
    }
}