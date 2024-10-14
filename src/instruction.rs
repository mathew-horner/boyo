use crate::gb::Register8;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    /// Not an instruction, this is the state that the CPU is in when it is
    /// first initialized.
    Initial,

    /// No operation. This instruction doesn’t do anything, but can be used to
    /// add a delay of one machine cycle and increment PC by one.
    NOP,

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

    /// `LD (HL), n`
    ///
    /// Load to the absolute address specified by the 16-bit register HL, the
    /// immediate data n.
    LD_HL_n { data: u8 },

    /// `JP nn`
    ///
    /// Unconditional jump to the absolute address specified by the 16-bit
    /// immediate operand nn.
    JP_nn { address: u16 },
}

impl Instruction {
    pub fn from_opcode(opcode: u8) -> Option<Self> {
        log::trace!("parse opcode 0x{opcode:02X}");
        match opcode {
            0x00 => Some(Self::NOP),
            0x01 => None,
            0x02 => None,
            0x03 => None,
            0x04 => None,
            0x05 => None,
            0x06 => Some(Self::LD_r_n { to: Register8::B }),
            0x07 => None,
            0x08 => None,
            0x09 => None,
            0x0A => None,
            0x0B => None,
            0x0C => None,
            0x0D => None,
            0x0E => Some(Self::LD_r_n { to: Register8::C }),
            0x0F => None,

            0x10 => None,
            0x11 => None,
            0x12 => None,
            0x13 => None,
            0x14 => None,
            0x15 => None,
            0x16 => Some(Self::LD_r_n { to: Register8::D }),
            0x17 => None,
            0x18 => None,
            0x19 => None,
            0x1A => None,
            0x1B => None,
            0x1C => None,
            0x1D => None,
            0x1E => Some(Self::LD_r_n { to: Register8::E }),
            0x1F => None,

            0x20 => None,
            0x21 => None,
            0x22 => None,
            0x23 => None,
            0x24 => None,
            0x25 => None,
            0x26 => Some(Self::LD_r_n { to: Register8::H }),
            0x27 => None,
            0x28 => None,
            0x29 => None,
            0x2A => None,
            0x2B => None,
            0x2C => None,
            0x2D => None,
            0x2E => Some(Self::LD_r_n { to: Register8::L }),
            0x2F => None,

            0x30 => None,
            0x31 => None,
            0x32 => None,
            0x33 => None,
            0x34 => None,
            0x35 => None,
            0x36 => Some(Self::LD_HL_n { data: 0 }),
            0x37 => None,
            0x38 => None,
            0x39 => None,
            0x3A => None,
            0x3B => None,
            0x3C => None,
            0x3D => None,
            0x3E => Some(Self::LD_r_n { to: Register8::A }),
            0x3F => None,

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

            0x70 => None,
            0x71 => None,
            0x72 => None,
            0x73 => None,
            0x74 => None,
            0x75 => None,
            0x76 => None,
            0x77 => None,
            0x78 => Some(Self::LD_r_r { to: Register8::A, from: Register8::B }),
            0x79 => Some(Self::LD_r_r { to: Register8::A, from: Register8::C }),
            0x7A => Some(Self::LD_r_r { to: Register8::A, from: Register8::D }),
            0x7B => Some(Self::LD_r_r { to: Register8::A, from: Register8::E }),
            0x7C => Some(Self::LD_r_r { to: Register8::A, from: Register8::H }),
            0x7D => Some(Self::LD_r_r { to: Register8::A, from: Register8::L }),
            0x7E => Some(Self::LD_r_HL { to: Register8::A }),
            0x7F => Some(Self::LD_r_r { to: Register8::A, from: Register8::A }),

            0x80 => None,
            0x81 => None,
            0x82 => None,
            0x83 => None,
            0x84 => None,
            0x85 => None,
            0x86 => None,
            0x87 => None,
            0x88 => None,
            0x89 => None,
            0x8A => None,
            0x8B => None,
            0x8C => None,
            0x8D => None,
            0x8E => None,
            0x8F => None,

            0x90 => None,
            0x91 => None,
            0x92 => None,
            0x93 => None,
            0x94 => None,
            0x95 => None,
            0x96 => None,
            0x97 => None,
            0x98 => None,
            0x99 => None,
            0x9A => None,
            0x9B => None,
            0x9C => None,
            0x9D => None,
            0x9E => None,
            0x9F => None,

            0xA0 => None,
            0xA1 => None,
            0xA2 => None,
            0xA3 => None,
            0xA4 => None,
            0xA5 => None,
            0xA6 => None,
            0xA7 => None,
            0xA8 => None,
            0xA9 => None,
            0xAA => None,
            0xAB => None,
            0xAC => None,
            0xAD => None,
            0xAE => None,
            0xAF => None,

            0xB0 => None,
            0xB1 => None,
            0xB2 => None,
            0xB3 => None,
            0xB4 => None,
            0xB5 => None,
            0xB6 => None,
            0xB7 => None,
            0xB8 => None,
            0xB9 => None,
            0xBA => None,
            0xBB => None,
            0xBC => None,
            0xBD => None,
            0xBE => None,
            0xBF => None,

            0xC0 => None,
            0xC1 => None,
            0xC2 => None,
            0xC3 => Some(Instruction::JP_nn { address: 0 }),
            0xC4 => None,
            0xC5 => None,
            0xC6 => None,
            0xC7 => None,
            0xC8 => None,
            0xC9 => None,
            0xCA => None,
            0xCB => None,
            0xCC => None,
            0xCD => None,
            0xCE => None,
            0xCF => None,

            0xD0 => None,
            0xD1 => None,
            0xD2 => None,
            0xD3 => None,
            0xD4 => None,
            0xD5 => None,
            0xD6 => None,
            0xD7 => None,
            0xD8 => None,
            0xD9 => None,
            0xDA => None,
            0xDB => None,
            0xDC => None,
            0xDD => None,
            0xDE => None,
            0xDF => None,

            0xE0 => None,
            0xE1 => None,
            0xE2 => None,
            0xE3 => None,
            0xE4 => None,
            0xE5 => None,
            0xE6 => None,
            0xE7 => None,
            0xE8 => None,
            0xE9 => None,
            0xEA => None,
            0xEB => None,
            0xEC => None,
            0xED => None,
            0xEE => None,
            0xEF => None,

            0xF0 => None,
            0xF1 => None,
            0xF2 => None,
            0xF3 => None,
            0xF4 => None,
            0xF5 => None,
            0xF6 => None,
            0xF7 => None,
            0xF8 => None,
            0xF9 => None,
            0xFA => None,
            0xFB => None,
            0xFC => None,
            0xFD => None,
            0xFE => None,
            0xFF => None,
        }
    }

    pub fn cycles(&self) -> usize {
        match self {
            Self::Initial => 0,
            Self::NOP | Self::LD_r_r { .. } => 1,
            Self::LD_r_n { .. } | Self::LD_r_HL { .. } | Self::LD_HL_r { .. } => 2,
            Self::LD_HL_n { .. } => 3,
            Self::JP_nn { .. } => 4,
        }
    }
}
