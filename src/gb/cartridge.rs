use std::path::Path;

pub struct Cartridge {
    pub rom: Vec<u8>,
}

const MAX_READ_BYTES: u16 = 4;

impl Cartridge {
    pub fn from<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
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

#[derive(Debug, thiserror::Error)]
pub enum CartridgeReadError {
    #[error("tried to read too many bytes at once ({count})")]
    CountTooHigh { count: u16 },
    #[error("tried to read from an address that is out of bounds")]
    OutOfBounds,
}
