pub struct Gameboy {
    pub cartridge: Option<GameboyCartridge>,
    pub cpu: LR35902,
}

impl Gameboy {
    pub fn new(cartridge: GameboyCartridge) -> Self {
        Self {
            cartridge: Some(cartridge),
            cpu: LR35902 { pc: 0x0100 },
        }
    }

    pub fn tick(&self) {
        println!("{}", self.cartridge.as_ref().unwrap().rom[self.cpu.pc as usize]);
    }
}

pub struct LR35902 {
    pub pc: u16,
}

pub struct GameboyCartridge {
    pub rom: Vec<u8>,
}

impl GameboyCartridge {
    pub fn from(path: &str) -> Result<Self, std::io::Error> {
        Ok(Self { rom: std::fs::read(path)? })
    }
}