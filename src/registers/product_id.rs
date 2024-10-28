use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Copy, Clone, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ProductId {
    Emc2305 = 0x34,
    Emc2303 = 0x35,
    Emc2302 = 0x36,
    Emc2301 = 0x37,
}

impl ProductId {
    const ADDRESS: u8 = 0xFD;

    /// Number of fans the device supports based on the Product ID.
    pub fn num_fans(&self) -> u8 {
        match self {
            ProductId::Emc2301 => 1,
            ProductId::Emc2302 => 2,
            ProductId::Emc2303 => 3,
            ProductId::Emc2305 => 5,
        }
    }
}

impl defmt::Format for ProductId {
    fn format(&self, f: defmt::Formatter) {
        match self {
            ProductId::Emc2305 => defmt::write!(f, "EMC2305"),
            ProductId::Emc2303 => defmt::write!(f, "EMC2303"),
            ProductId::Emc2302 => defmt::write!(f, "EMC2302"),
            ProductId::Emc2301 => defmt::write!(f, "EMC2301"),
        }
    }
}
