pub struct Cartridge {
    pub rom: Vec<u8>,
}

impl Cartridge {
    pub fn from(path: &str) -> Result<Self, std::io::Error> {
        Ok(Self { rom: std::fs::read(path)? })
    }

    // TODO: count should have a maximum value of 4.
    pub fn read_bytes(&self, address: u16, count: u16) -> u32 {
        let mut data: u32 = 0;
        for i in 0..count {
            data <<= 8;
            data |= (self.rom[(address + i) as usize]) as u32;
        }
        data
    }
}