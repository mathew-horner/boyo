pub struct Gameboy {
    pub cartridge: Option<Cartridge>,
    pub cpu: LR35902,
}

impl Gameboy {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge: Some(cartridge),
            cpu: LR35902 { pc: 0x0100, sp: 0xFFFE, a: 0, b: 0, c: 0, d: 0, e: 0, f: 0, h: 0, l: 0 },
        }
    }

    pub fn tick(&mut self) -> u8 {
        if self.cartridge.is_none() {
            return 0;
        }
        // PERF: check if as_ref() has a large amount of overhead?
        let cartridge = self.cartridge.as_ref().unwrap();
        let opcode = cartridge.read_bytes(self.cpu.pc, 1);
        let opcode = match Opcode::parse(opcode) {
            Some(op) => op,
            None => {
                println!("Error: encountered bad opcode {:#X} at address: {:#X}", opcode, self.cpu.pc);
                std::process::exit(0);
            }
        };
        // PERF: avoid re-reading memory here, but it's the simplest solution atm.
        let instruction = cartridge.read_bytes(self.cpu.pc, opcode.size());
        match opcode {
            Opcode::NOP => {
                self.cpu.pc += 1;
                4
            }
            Opcode::JPA16 => {
                self.cpu.pc = (instruction & 0xFFFF) as u16;
                16
            },
        }
    }
}

pub struct LR35902 {
    pub pc: u16,
    pub sp: u16,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
}

pub enum Opcode {
    NOP,
    JPA16,
}

impl Opcode {
    fn parse(opcode: u32) -> Option<Self> {
        if opcode == 0 {
            return Some(Opcode::NOP);
        }
        if opcode == 0xC3 {
            return Some(Opcode::JPA16);
        }
        None
    }

    // TODO: Use global HashMap.
    fn size(&self) -> u16 {
        match self {
            Opcode::NOP => 1,
            Opcode::JPA16 => 3,
        }
    }
}

pub struct Cartridge {
    pub rom: Vec<u8>,
}

impl Cartridge {
    pub fn from(path: &str) -> Result<Self, std::io::Error> {
        Ok(Self { rom: std::fs::read(path)? })
    }

    pub fn read_bytes(&self, address: u16, count: u16) -> u32 {
        let mut data: u32 = 0;
        for i in 0..count {
            data <<= 8;
            data |= (self.rom[(address + i) as usize]) as u32;
        }
        data
    }
}