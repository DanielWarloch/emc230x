#[derive(Debug)]
pub enum Error {
    /// I2C bus error
    I2c,

    /// Invalid fan number
    InvalidFan,
}
