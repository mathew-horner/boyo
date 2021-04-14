mod cartridge;
mod cpu;
mod opcode;
pub use cartridge::Cartridge;
use cpu::LR35902;
use opcode::Opcode;

const MEMORY_SIZE: usize = 8192;

pub struct Gameboy {
    pub cartridge: Option<Cartridge>,
    pub cpu: LR35902,
    pub memory: [u8; MEMORY_SIZE],
}

impl Gameboy {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge: Some(cartridge),
            cpu: LR35902 { pc: 0x0100, sp: 0xFFFE, a: 0, b: 0, c: 0, d: 0, e: 0, f: 0, h: 0, l: 0 },
            memory: [0; MEMORY_SIZE],
        }
    }

    pub fn tick(&mut self) -> u8 {
        if self.cartridge.is_none() {
            return 0;
        }
        // TODO-PERF: check if as_ref() has a large amount of overhead?
        let cartridge = self.cartridge.as_ref().unwrap();
        let opcode = cartridge.read_bytes(self.cpu.pc, 1) as u8;
        let size = Opcode::size(opcode) as u16;
        let opcode = match Opcode::parse(opcode) {
            Some(op) => op,
            None => {
                println!("Error: encountered bad opcode {:#X} at address: {:#X}", opcode, self.cpu.pc);
                std::process::exit(0);
            }
        };
        // TODO-PERF: avoid re-reading memory here, but it's the simplest solution atm.
        let instruction = cartridge.read_bytes(self.cpu.pc, size);
        let mut skip_pc = false;
        let cycles = match opcode {
            Opcode::NOP => 4,
            Opcode::RLCA => {
                // TODO-Q: Is this the right thing to do? How do we set Z?
                self.cpu.set_flags(1, 0, 0, self.cpu.z());
                4
            },
            Opcode::LD_HL_B => {
                self.memory[self.cpu.hl() as usize] = self.cpu.b;
                8
            },
            Opcode::JP_a16 => {
                self.cpu.pc = (instruction & 0xFFFF) as u16;
                skip_pc = true;
                16
            },
        };
        if !skip_pc {
            self.cpu.pc += size;
        }
        cycles
    }
}