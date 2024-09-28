use std::fmt;

pub struct Cartridge {
    pub rom: Vec<u8>,
}

const MAX_READ_BYTES: u16 = 4;

impl Cartridge {
    pub fn from(path: &str) -> Result<Self, std::io::Error> {
        Ok(Self { rom: std::fs::read(path)? })
    }

    pub fn read_bytes(&self, address: u16, count: u16) -> Result<u32, CartridgeReadError> {
        // TODO: This might be unnecessary because we statically define all opcode sizes
        // and none will be greater than 4.
        if count > MAX_READ_BYTES {
            return Err(CartridgeReadError::CountTooHigh { count });
        }
        if (address + count - 1) as usize >= self.rom.len() {
            return Err(CartridgeReadError::OutOfBounds);
        }
        let mut data: u32 = 0;
        for i in 0..count {
            data <<= 8;
            data |= (self.rom[(address + i) as usize]) as u32;
        }
        Ok(data)
    }
}

#[derive(Debug)]
pub enum CartridgeReadError {
    CountTooHigh { count: u16 },
    OutOfBounds,
}

impl fmt::Display for CartridgeReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::CountTooHigh { count } =>
                format!("Tried to read too many bytes at once! ({})", count),
            Self::OutOfBounds => "Tried to read from address that is out of bounds!".to_string(),
        })
    }
}
