const INITIAL_PC: u16 = 0x0100;
const INITIAL_SP: u16 = 0xFFFE;

pub struct Gameboy {
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

    instruction_state: InstructionState,
}

impl Gameboy {
    pub fn new() -> Self {
        Self {
            pc: INITIAL_PC,
            sp: INITIAL_SP,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            rom: Vec::new(),
            instruction_state: InstructionState::default(),
        }
    }

    pub fn execute(self) {}

    pub fn cycle(&mut self) {
        let byte = self.rom[self.pc as usize];
        self.pc += 1;

        if self.instruction_state.is_done() {
            let instruction = Instruction::from_opcode(byte).expect("invalid opcode");
            self.instruction_state = InstructionState { instruction, m_cycle: 0 };
        }

        self.instruction_state.m_cycle += 1;

        match self.instruction_state.instruction {
            Instruction::Initial => unreachable!(
                "Initial should return true for is_done and this branch should never be reached"
            ),
            Instruction::LD_r_r { to, from } => {
                if to != from {
                    *self.register8_mut(to) = self.register8(from);
                }
            },
            Instruction::LD_r_n { to } => {
                if self.instruction_state.m_cycle == 2 {
                    *self.register8_mut(to) = byte;
                }
            },
            Instruction::LD_r_HL { to } => {
                if self.instruction_state.m_cycle == 2 {
                    let hl = self.register16(Register16::HL);
                    *self.register8_mut(to) = self.read_memory(hl);
                }
            },
            Instruction::LD_HL_r { from } => {
                if self.instruction_state.m_cycle == 2 {
                    let hl = self.register16(Register16::HL);
                    let r = self.register8(from);
                    self.write_memory(hl, r);
                }
            },
        }
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

    fn read_memory(&self, address: u16) -> u8 {
        0
    }

    fn write_memory(&mut self, address: u16, data: u8) {}
}

struct InstructionState {
    instruction: Instruction,
    m_cycle: usize,
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

#[derive(Copy, Clone, Eq, PartialEq)]
enum Register8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

enum Register16 {
    AF,
    BC,
    DE,
    HL,
}

#[allow(non_camel_case_types)]
enum Instruction {
    /// Not an instruction, this is the state that the CPU is in when it is
    /// first initialized.
    Initial,

    /// `LD r, r'`
    ///
    /// Load to the 8-bit register `r`, data from the 8-bit register `r'`.
    LD_r_r { to: Register8, from: Register8 },

    /// `LD r, n`
    ///
    /// Load to the 8-bit register `r`, the immediate data `n`.
    LD_r_n { to: Register8 },

    /// `LD r, (HL)`
    ///
    /// Load to the 8-bit register `r`, data from the absolute address specified
    /// by the 16-bit register `HL`.
    LD_r_HL { to: Register8 },

    /// `LD (HL), r`
    ///
    /// Load to the absolute address specified by the 16-bit register `HL`, data
    /// from the 8-bit register `r`.
    LD_HL_r { from: Register8 },
}

impl Instruction {
    fn from_opcode(opcode: u8) -> Option<Self> {
        match opcode {
            0x00 => todo!(),
            0x01 => todo!(),
            0x02 => todo!(),
            0x03 => todo!(),
            0x04 => todo!(),
            0x05 => todo!(),
            0x06 => Some(Self::LD_r_n { to: Register8::B }),
            0x07 => todo!(),
            0x08 => todo!(),
            0x09 => todo!(),
            0x0A => todo!(),
            0x0B => todo!(),
            0x0C => todo!(),
            0x0D => todo!(),
            0x0E => Some(Self::LD_r_n { to: Register8::C }),
            0x0F => todo!(),

            0x10 => todo!(),
            0x11 => todo!(),
            0x12 => todo!(),
            0x13 => todo!(),
            0x14 => todo!(),
            0x15 => todo!(),
            0x16 => Some(Self::LD_r_n { to: Register8::D }),
            0x17 => todo!(),
            0x18 => todo!(),
            0x19 => todo!(),
            0x1A => todo!(),
            0x1B => todo!(),
            0x1C => todo!(),
            0x1D => todo!(),
            0x1E => Some(Self::LD_r_n { to: Register8::E }),
            0x1F => todo!(),

            0x20 => todo!(),
            0x21 => todo!(),
            0x22 => todo!(),
            0x23 => todo!(),
            0x24 => todo!(),
            0x25 => todo!(),
            0x26 => Some(Self::LD_r_n { to: Register8::H }),
            0x27 => todo!(),
            0x28 => todo!(),
            0x29 => todo!(),
            0x2A => todo!(),
            0x2B => todo!(),
            0x2C => todo!(),
            0x2D => todo!(),
            0x2E => Some(Self::LD_r_n { to: Register8::L }),
            0x2F => todo!(),

            0x30 => todo!(),
            0x31 => todo!(),
            0x32 => todo!(),
            0x33 => todo!(),
            0x34 => todo!(),
            0x35 => todo!(),
            0x36 => todo!(),
            0x37 => todo!(),
            0x38 => todo!(),
            0x39 => todo!(),
            0x3A => todo!(),
            0x3B => todo!(),
            0x3C => todo!(),
            0x3D => todo!(),
            0x3E => Some(Self::LD_r_n { to: Register8::A }),
            0x3F => todo!(),

            0x40 => Some(Self::LD_r_r { to: Register8::B, from: Register8::B }),
            0x41 => Some(Self::LD_r_r { to: Register8::B, from: Register8::C }),
            0x42 => Some(Self::LD_r_r { to: Register8::B, from: Register8::D }),
            0x43 => Some(Self::LD_r_r { to: Register8::B, from: Register8::E }),
            0x44 => Some(Self::LD_r_r { to: Register8::B, from: Register8::H }),
            0x45 => Some(Self::LD_r_r { to: Register8::B, from: Register8::L }),
            0x46 => Some(Self::LD_r_HL { to: Register8::B }),
            0x47 => Some(Self::LD_r_r { to: Register8::B, from: Register8::A }),
            0x48 => Some(Self::LD_r_r { to: Register8::C, from: Register8::B }),
            0x49 => Some(Self::LD_r_r { to: Register8::C, from: Register8::C }),
            0x4A => Some(Self::LD_r_r { to: Register8::C, from: Register8::D }),
            0x4B => Some(Self::LD_r_r { to: Register8::C, from: Register8::E }),
            0x4C => Some(Self::LD_r_r { to: Register8::C, from: Register8::H }),
            0x4D => Some(Self::LD_r_r { to: Register8::C, from: Register8::L }),
            0x4E => Some(Self::LD_r_HL { to: Register8::C }),
            0x4F => Some(Self::LD_r_r { to: Register8::C, from: Register8::A }),

            0x50 => Some(Self::LD_r_r { to: Register8::D, from: Register8::B }),
            0x51 => Some(Self::LD_r_r { to: Register8::D, from: Register8::C }),
            0x52 => Some(Self::LD_r_r { to: Register8::D, from: Register8::D }),
            0x53 => Some(Self::LD_r_r { to: Register8::D, from: Register8::E }),
            0x54 => Some(Self::LD_r_r { to: Register8::D, from: Register8::H }),
            0x55 => Some(Self::LD_r_r { to: Register8::D, from: Register8::L }),
            0x56 => Some(Self::LD_r_HL { to: Register8::D }),
            0x57 => Some(Self::LD_r_r { to: Register8::D, from: Register8::A }),
            0x58 => Some(Self::LD_r_r { to: Register8::E, from: Register8::B }),
            0x59 => Some(Self::LD_r_r { to: Register8::E, from: Register8::C }),
            0x5A => Some(Self::LD_r_r { to: Register8::E, from: Register8::D }),
            0x5B => Some(Self::LD_r_r { to: Register8::E, from: Register8::E }),
            0x5C => Some(Self::LD_r_r { to: Register8::E, from: Register8::H }),
            0x5D => Some(Self::LD_r_r { to: Register8::E, from: Register8::L }),
            0x5E => Some(Self::LD_r_HL { to: Register8::E }),
            0x5F => Some(Self::LD_r_r { to: Register8::E, from: Register8::A }),

            0x60 => Some(Self::LD_r_r { to: Register8::H, from: Register8::B }),
            0x61 => Some(Self::LD_r_r { to: Register8::H, from: Register8::C }),
            0x62 => Some(Self::LD_r_r { to: Register8::H, from: Register8::D }),
            0x63 => Some(Self::LD_r_r { to: Register8::H, from: Register8::E }),
            0x64 => Some(Self::LD_r_r { to: Register8::H, from: Register8::H }),
            0x65 => Some(Self::LD_r_r { to: Register8::H, from: Register8::L }),
            0x66 => Some(Self::LD_r_HL { to: Register8::H }),
            0x67 => Some(Self::LD_r_r { to: Register8::H, from: Register8::A }),
            0x68 => Some(Self::LD_r_r { to: Register8::L, from: Register8::B }),
            0x69 => Some(Self::LD_r_r { to: Register8::L, from: Register8::C }),
            0x6A => Some(Self::LD_r_r { to: Register8::L, from: Register8::D }),
            0x6B => Some(Self::LD_r_r { to: Register8::L, from: Register8::E }),
            0x6C => Some(Self::LD_r_r { to: Register8::L, from: Register8::H }),
            0x6D => Some(Self::LD_r_r { to: Register8::L, from: Register8::L }),
            0x6E => Some(Self::LD_r_HL { to: Register8::L }),
            0x6F => Some(Self::LD_r_r { to: Register8::L, from: Register8::A }),

            0x70 => todo!(),
            0x71 => todo!(),
            0x72 => todo!(),
            0x73 => todo!(),
            0x74 => todo!(),
            0x75 => todo!(),
            0x76 => todo!(),
            0x77 => todo!(),
            0x78 => Some(Self::LD_r_r { to: Register8::A, from: Register8::B }),
            0x79 => Some(Self::LD_r_r { to: Register8::A, from: Register8::C }),
            0x7A => Some(Self::LD_r_r { to: Register8::A, from: Register8::D }),
            0x7B => Some(Self::LD_r_r { to: Register8::A, from: Register8::E }),
            0x7C => Some(Self::LD_r_r { to: Register8::A, from: Register8::H }),
            0x7D => Some(Self::LD_r_r { to: Register8::A, from: Register8::L }),
            0x7E => Some(Self::LD_r_HL { to: Register8::A }),
            0x7F => Some(Self::LD_r_r { to: Register8::A, from: Register8::A }),

            _ => None,
        }
    }

    fn cycles(&self) -> usize {
        match self {
            Self::Initial => 0,
            Self::LD_r_r { .. } => 1,
            Self::LD_r_n { .. } | Self::LD_r_HL { .. } | Self::LD_HL_r { .. } => 2,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn register16_combines_8bit_registers() {
        let mut gb = Gameboy::new();

        gb.a = 0x20;
        gb.f = 0x94;
        assert_eq!(gb.register16(Register16::AF), 0x2094);

        gb.b = 0x42;
        gb.c = 0x21;
        assert_eq!(gb.register16(Register16::BC), 0x4221);

        gb.d = 0x65;
        gb.e = 0xBC;
        assert_eq!(gb.register16(Register16::DE), 0x65BC);

        gb.h = 0x0A;
        gb.l = 0xF0;
        assert_eq!(gb.register16(Register16::HL), 0x0AF0);
    }
}
