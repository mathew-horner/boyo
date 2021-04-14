use super::common::Emulator;

pub struct GameboyAdvance {
    pub cartridge: Option<GameboyAdvanceCartridge>,
    pub cpu: ARM7TDMI,
}

impl GameboyAdvance {
    pub fn new(cartridge: GameboyAdvanceCartridge) -> Self {
        Self {
            cartridge: Some(cartridge),
            cpu: ARM7TDMI { pc: 0 },
        }
    }
}

impl Emulator for GameboyAdvance {
    fn start(&self) {
        #[allow(while_true)]
        while true { }
    }
}

pub struct ARM7TDMI {
    pub pc: u16,
}

pub struct GameboyAdvanceCartridge {
    pub rom: Vec<u8>,
}

impl GameboyAdvanceCartridge {
    pub fn from(path: &str) -> Result<Self, std::io::Error> {
        Ok(Self { rom: std::fs::read(path)? })
    }
}