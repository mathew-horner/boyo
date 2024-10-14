use std::fmt::UpperHex;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::instruction::Instruction;

pub const INITIAL_PC: u16 = 0x0100;

const INITIAL_SP: u16 = 0xFFFE;

pub struct Gameboy {
    system: System,
    instruction_state: InstructionState,
}

impl Gameboy {
    #[cfg(test)]
    fn no_cartridge() -> Self {
        Self::new(Vec::new())
    }

    pub fn new(rom: Vec<u8>) -> Self {
        Self { system: System::new(rom), instruction_state: InstructionState::default() }
    }

    pub fn execute(mut self) -> ! {
        const CYCLES_PER_FRAME: u32 = 69905;
        const REFRESH_RATE: f64 = 60.0;

        let frame_duration = Duration::from_secs_f64(1.0 / REFRESH_RATE);

        loop {
            let start = Instant::now();
            for _ in 0..CYCLES_PER_FRAME {
                self.cycle();
            }
            // TODO: Actually draw frame.
            println!("Draw frame");
            sleep(frame_duration.checked_sub(start.elapsed()).unwrap_or(Duration::ZERO));
        }
    }

    pub fn cycle(&mut self) {
        if self.instruction_state.is_done() {
            let byte = self.system.fetch();
            let instruction = Instruction::from_opcode(byte).expect("invalid opcode");
            self.instruction_state = InstructionState { instruction, m_cycle: 0 };
        }

        self.instruction_state.m_cycle += 1;

        match &mut self.instruction_state.instruction {
            Instruction::Initial => unreachable!(
                "Initial should return true for is_done and this branch should never be reached"
            ),
            Instruction::NOP => {},
            Instruction::LD_r_r { to, from } => {
                if to != from {
                    *self.system.register8_mut(*to) = self.system.register8(*from);
                }
            },
            Instruction::LD_r_n { to } => {
                if self.instruction_state.m_cycle == 2 {
                    *self.system.register8_mut(*to) = self.system.fetch();
                }
            },
            Instruction::LD_r_HL { to } => {
                if self.instruction_state.m_cycle == 2 {
                    let hl = self.system.register16(Register16::HL);
                    *self.system.register8_mut(*to) = self.system.random_access(hl);
                }
            },
            Instruction::LD_HL_r { from } => {
                if self.instruction_state.m_cycle == 2 {
                    let hl = self.system.register16(Register16::HL);
                    let r = self.system.register8(*from);
                    self.system.write_memory(hl, r);
                }
            },
            Instruction::LD_HL_n { data } => match self.instruction_state.m_cycle {
                1 => {},
                2 => {
                    *data = self.system.fetch();
                },
                3 => {
                    let hl = self.system.register16(Register16::HL);
                    self.system.write_memory(hl, *data);
                },
                _ => unreachable!(),
            },
            Instruction::JP_nn { address } => match self.instruction_state.m_cycle {
                1 => {},
                2 => {
                    *address = self.system.fetch() as u16;
                },
                3 => {
                    *address |= (self.system.fetch() as u16) << 8;
                },
                4 => {
                    self.system.pc = *address;
                },
                _ => unreachable!(),
            },
        }
    }

    // If the next cycle is to read an opcode that is unrecognized (read:
    // unimplemented for now), then return the opcode as a byte so it can be
    // displayed in an error message to the user without panicking.
    pub fn peek_instruction_state(&self) -> Result<InstructionState, u8> {
        let mut state = self.instruction_state.clone();
        if state.is_done() {
            let byte = self.system.rom[self.system.pc as usize];
            let instruction = Instruction::from_opcode(byte).ok_or(byte)?;
            state = InstructionState { instruction, m_cycle: 0 };
        }
        state.m_cycle += 1;
        Ok(state)
    }

    pub fn registers<'a>(&'a self) -> Registers<'a> {
        Registers { system: &self.system, idx: 0 }
    }

    pub fn pc(&self) -> u16 {
        self.system.pc
    }
}

struct System {
    pc: u16,
    sp: u16,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,

    rom: Vec<u8>,
}

impl System {
    fn new(rom: Vec<u8>) -> Self {
        Self { pc: INITIAL_PC, sp: INITIAL_SP, a: 0, b: 0, c: 0, d: 0, e: 0, f: 0, h: 0, l: 0, rom }
    }

    fn register8(&self, register: Register8) -> u8 {
        match register {
            Register8::A => self.a,
            Register8::B => self.b,
            Register8::C => self.c,
            Register8::D => self.d,
            Register8::E => self.e,
            Register8::F => self.f,
            Register8::H => self.h,
            Register8::L => self.l,
        }
    }

    fn register8_mut(&mut self, register: Register8) -> &mut u8 {
        match register {
            Register8::A => &mut self.a,
            Register8::B => &mut self.b,
            Register8::C => &mut self.c,
            Register8::D => &mut self.d,
            Register8::E => &mut self.e,
            Register8::F => &mut self.f,
            Register8::H => &mut self.h,
            Register8::L => &mut self.l,
        }
    }

    fn register16(&self, register: Register16) -> u16 {
        match register {
            Register16::AF => (self.a as u16) << 8 | self.f as u16,
            Register16::BC => (self.b as u16) << 8 | self.c as u16,
            Register16::DE => (self.d as u16) << 8 | self.e as u16,
            Register16::HL => (self.h as u16) << 8 | self.l as u16,
        }
    }

    // TODO: Implement memory I/O.

    fn fetch(&mut self) -> u8 {
        let byte = self.rom[self.pc as usize];
        self.pc += 1;
        byte
    }

    fn random_access(&self, address: u16) -> u8 {
        0
    }

    fn write_memory(&mut self, address: u16, data: u8) {}
}

#[derive(Clone, Debug)]
pub struct InstructionState {
    pub instruction: Instruction,
    pub m_cycle: usize,
}

impl Default for InstructionState {
    fn default() -> Self {
        Self { instruction: Instruction::Initial, m_cycle: 0 }
    }
}

impl InstructionState {
    fn is_done(&self) -> bool {
        let instruction_cycles = self.instruction.cycles();
        debug_assert!(self.m_cycle <= instruction_cycles);
        self.m_cycle == instruction_cycles
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
}

pub struct Registers<'a> {
    system: &'a System,
    idx: usize,
}

impl<'a> Iterator for Registers<'a> {
    type Item = Register;

    fn next(&mut self) -> Option<Self::Item> {
        let register = match self.idx {
            0 => Register { name: "pc", value: RegisterValue::U16(self.system.pc) },
            1 => Register { name: "sp", value: RegisterValue::U16(self.system.sp) },
            2 => Register { name: "a", value: RegisterValue::U8(self.system.a) },
            3 => Register { name: "b", value: RegisterValue::U8(self.system.b) },
            4 => Register { name: "c", value: RegisterValue::U8(self.system.c) },
            5 => Register { name: "d", value: RegisterValue::U8(self.system.d) },
            6 => Register { name: "e", value: RegisterValue::U8(self.system.e) },
            7 => Register { name: "f", value: RegisterValue::U8(self.system.f) },
            8 => Register { name: "h", value: RegisterValue::U8(self.system.h) },
            9 => Register { name: "l", value: RegisterValue::U8(self.system.l) },
            _ => return None,
        };
        self.idx += 1;
        Some(register)
    }
}

pub struct Register {
    pub name: &'static str,
    pub value: RegisterValue,
}

pub enum RegisterValue {
    U8(u8),
    U16(u16),
}

impl UpperHex for RegisterValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U8(value) => write!(f, "0x{value:02X}"),
            Self::U16(value) => write!(f, "0x{value:04X}"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn register16_combines_8bit_registers() {
        let mut gb = Gameboy::no_cartridge();

        gb.system.a = 0x20;
        gb.system.f = 0x94;
        assert_eq!(gb.system.register16(Register16::AF), 0x2094);

        gb.system.b = 0x42;
        gb.system.c = 0x21;
        assert_eq!(gb.system.register16(Register16::BC), 0x4221);

        gb.system.d = 0x65;
        gb.system.e = 0xBC;
        assert_eq!(gb.system.register16(Register16::DE), 0x65BC);

        gb.system.h = 0x0A;
        gb.system.l = 0xF0;
        assert_eq!(gb.system.register16(Register16::HL), 0x0AF0);
    }
}
