mod cartridge;
mod cpu;
mod opcode;
pub use cartridge::Cartridge;
use cpu::LR35902;
use opcode::Opcode;
use std::fmt;

const _8KB: usize = 8192;

pub struct Gameboy {
    pub cartridge: Option<Cartridge>,
    pub cpu: LR35902,
    pub ram: [u8; _8KB],
    pub vram: [u8; _8KB],
}

impl Gameboy {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge: Some(cartridge),
            cpu: LR35902 { pc: 0x0100, sp: 0xFFFE, a: 0, b: 0, c: 0, d: 0, e: 0, f: 0, h: 0, l: 0 },
            ram: [0; _8KB],
            vram: [0; _8KB],
        }
    }

    pub fn tick(&mut self) -> Result<u8, TickError> {
        if self.cartridge.is_none() {
            return Err(TickError::NoCartridge);
        }
        // TODO-PERF: check if as_ref() has a large amount of overhead?
        let cartridge = self.cartridge.as_ref().unwrap();
        let opcode = cartridge.read_bytes(self.cpu.pc, 1) as u8;
        let base_cycles = Opcode::base_cycles(opcode);
        let size = Opcode::size(opcode) as u16;
        let opcode = match Opcode::parse(opcode) {
            Some(op) => op,
            None => {
                return Err(TickError::InvalidOpcode { opcode, address: self.cpu.pc });
            }
        };
        // TODO-PERF: avoid re-reading memory here, but it's the simplest solution atm.
        let instruction = cartridge.read_bytes(self.cpu.pc, size);
        let mut skip_pc = false;
        match opcode {
            Opcode::NOP => (),
            Opcode::LD_BC_d16 => {
                self.cpu.set_bc((instruction & 0xFFFF) as u16);
            },
            Opcode::LD_BC_A => {
                self.try_write(self.cpu.bc(), self.cpu.a)?;
            },
            Opcode::INC_BC => {
            },
            Opcode::INC_B => {
            },
            Opcode::DEC_B => {
            },
            Opcode::LD_B_d8 => {
                self.cpu.b = (instruction & 0xFF) as u8;
            },
            Opcode::RLCA => {
                // TODO-Q: Is this the right thing to do? How do we set Z?
                self.cpu.set_flags(1, 0, 0, self.cpu.z());
            },
            Opcode::LD_a16_SP => {
            },
            Opcode::ADD_HL_BC => {
            },
            Opcode::LD_A_BC => {
                self.cpu.a = self.try_read(self.cpu.bc())?;
            },
            Opcode::DEC_BC => {
            },
            Opcode::INC_C => {
            },
            Opcode::DEC_C => {
            },
            Opcode::LD_C_d8 => {
                self.cpu.c = (instruction & 0xFF) as u8;
            },
            Opcode::RRCA => {
            },
            Opcode::STOP_0 => {
            },
            Opcode::LD_DE_d16 => {
            },
            Opcode::LD_DE_A => {
                self.try_write(self.cpu.de(), self.cpu.a)?;
            },
            Opcode::INC_DE => {
            },
            Opcode::INC_D => {
            },
            Opcode::DEC_D => {
            },
            Opcode::LD_D_d8 => {
                self.cpu.d = (instruction & 0xFF) as u8;
            },
            Opcode::RLA => {
            },
            Opcode::JR_r8 => {
            },
            Opcode::ADD_HL_DE => {
            },
            Opcode::LD_A_DE => {
                self.cpu.a = self.try_read(self.cpu.de())?;
            },
            Opcode::DEC_DE => {
            },
            Opcode::INC_E => {
            },
            Opcode::DEC_E => {
            },
            Opcode::LD_E_d8 => {
                self.cpu.e = (instruction & 0xFF) as u8;
            },
            Opcode::RRA => {
            },
            Opcode::JR_NZ_r8 => {
            },
            Opcode::LD_HL_d16 => {
            },
            Opcode::LD_HL_plus_A => {
            },
            Opcode::INC_HL => {
            },
            Opcode::INC_H => {
            },
            Opcode::DEC_H => {
            },
            Opcode::LD_H_d8 => {
                self.cpu.h = (instruction & 0xFF) as u8;
            },
            Opcode::DAA => {
            },
            Opcode::JR_Z_r8 => {
            },
            Opcode::ADD_HL_HL => {
            },
            Opcode::LD_A_HL_ => {
            },
            Opcode::DEC_HL => {
            },
            Opcode::INC_L => {
            },
            Opcode::DEC_L => {
            },
            Opcode::LD_L_d8 => {
                self.cpu.l = (instruction & 0xFF) as u8;
            },
            Opcode::CPL => {
            },
            Opcode::JR_NC_r8 => {
            },
            Opcode::LD_SP_d16 => {
            },
            Opcode::LD_HL_minus_A => {
            },
            Opcode::INC_SP => {
            },
            Opcode::INC_atHL => {
            },
            Opcode::DEC_atHL => {
            },
            Opcode::LD_HL_d8 => {
            },
            Opcode::SCF => {
            },
            Opcode::JR_C_r8 => {
            },
            Opcode::ADD_HL_SP => {
            },
            Opcode::LD_A_HL_minus => {
            },
            Opcode::DEC_SP => {
            },
            Opcode::INC_A => {
            },
            Opcode::DEC_A => {
            },
            Opcode::LD_A_d8 => {
            },
            Opcode::CCF => {
            },
            Opcode::LD_B_B => (),
            Opcode::LD_B_C => {
                self.cpu.b = self.cpu.c;
            },
            Opcode::LD_B_D => {
                self.cpu.b = self.cpu.d;
            },
            Opcode::LD_B_E => {
                self.cpu.b = self.cpu.e;
            },
            Opcode::LD_B_H => {
                self.cpu.b = self.cpu.h;
            },
            Opcode::LD_B_L => {
                self.cpu.b = self.cpu.l;
            },
            Opcode::LD_B_HL => {
                self.cpu.b = self.try_read(self.cpu.hl())?;
            },
            Opcode::LD_B_A => {
                self.cpu.b = self.cpu.a;
            },
            Opcode::LD_C_B => {
            },
            Opcode::LD_C_C => {
            },
            Opcode::LD_C_D => {
            },
            Opcode::LD_C_E => {
            },
            Opcode::LD_C_H => {
            },
            Opcode::LD_C_L => {
            },
            Opcode::LD_C_HL => {
            },
            Opcode::LD_C_A => {
                self.cpu.c = self.cpu.a;
            },
            Opcode::LD_D_B => {
            },
            Opcode::LD_D_C => {
            },
            Opcode::LD_D_D => {
            },
            Opcode::LD_D_E => {
            },
            Opcode::LD_D_H => {
            },
            Opcode::LD_D_L => {
            },
            Opcode::LD_D_HL => {
            },
            Opcode::LD_D_A => {
                self.cpu.d = self.cpu.a;
            },
            Opcode::LD_E_B => {
            },
            Opcode::LD_E_C => {
            },
            Opcode::LD_E_D => {
            },
            Opcode::LD_E_E => {
            },
            Opcode::LD_E_H => {
            },
            Opcode::LD_E_L => {
            },
            Opcode::LD_E_HL => {
            },
            Opcode::LD_E_A => {
                self.cpu.e = self.cpu.a;
            },
            Opcode::LD_H_B => {
            },
            Opcode::LD_H_C => {
            },
            Opcode::LD_H_D => {
            },
            Opcode::LD_H_E => {
            },
            Opcode::LD_H_H => {
            },
            Opcode::LD_H_L => {
            },
            Opcode::LD_H_HL => {
            },
            Opcode::LD_H_A => {
                self.cpu.h = self.cpu.a;
            },
            Opcode::LD_L_B => {
            },
            Opcode::LD_L_C => {
            },
            Opcode::LD_L_D => {
            },
            Opcode::LD_L_E => {
            },
            Opcode::LD_L_H => {
            },
            Opcode::LD_L_L => {
            },
            Opcode::LD_L_HL => {
            },
            Opcode::LD_L_A => {
                self.cpu.l = self.cpu.a;
            },
            Opcode::LD_HL_B => {
                self.try_write(self.cpu.hl(), self.cpu.b)?;
            },
            Opcode::LD_HL_C => {
            },
            Opcode::LD_HL_D => {
            },
            Opcode::LD_HL_E => {
            },
            Opcode::LD_HL_H => {
            },
            Opcode::LD_HL_L => {
            },
            Opcode::HALT => {
            },
            Opcode::LD_HL_A => {
                self.try_write(self.cpu.hl(), self.cpu.a)?;
            },
            Opcode::LD_A_B => {
                self.cpu.a = self.cpu.b;
            },
            Opcode::LD_A_C => {
                self.cpu.a = self.cpu.c;
            },
            Opcode::LD_A_D => {
                self.cpu.a = self.cpu.d;
            },
            Opcode::LD_A_E => {
                self.cpu.a = self.cpu.e;
            },
            Opcode::LD_A_H => {
                self.cpu.a = self.cpu.h;
            },
            Opcode::LD_A_L => {
                self.cpu.a = self.cpu.l;
            },
            Opcode::LD_A_HL => {
                self.cpu.a = self.try_read(self.cpu.hl())?;
            },
            Opcode::LD_A_A => (),
            Opcode::ADD_A_B => {
            },
            Opcode::ADD_A_C => {
            },
            Opcode::ADD_A_D => {
            },
            Opcode::ADD_A_E => {
            },
            Opcode::ADD_A_H => {
            },
            Opcode::ADD_A_L => {
            },
            Opcode::ADD_A_HL => {
            },
            Opcode::ADD_A_A => {
            },
            Opcode::ADC_A_B => {
            },
            Opcode::ADC_A_C => {
            },
            Opcode::ADC_A_D => {
            },
            Opcode::ADC_A_E => {
            },
            Opcode::ADC_A_H => {
            },
            Opcode::ADC_A_L => {
            },
            Opcode::ADC_A_HL => {
            },
            Opcode::ADC_A_A => {
            },
            Opcode::SUB_B => {
            },
            Opcode::SUB_C => {
            },
            Opcode::SUB_D => {
            },
            Opcode::SUB_E => {
            },
            Opcode::SUB_H => {
            },
            Opcode::SUB_L => {
            },
            Opcode::SUB_HL => {
            },
            Opcode::SUB_A => {
            },
            Opcode::SBC_A_B => {
            },
            Opcode::SBC_A_C => {
            },
            Opcode::SBC_A_D => {
            },
            Opcode::SBC_A_E => {
            },
            Opcode::SBC_A_H => {
            },
            Opcode::SBC_A_L => {
            },
            Opcode::SBC_A_HL => {
            },
            Opcode::SBC_A_A => {
            },
            Opcode::AND_B => {
            },
            Opcode::AND_C => {
            },
            Opcode::AND_D => {
            },
            Opcode::AND_E => {
            },
            Opcode::AND_H => {
            },
            Opcode::AND_L => {
            },
            Opcode::AND_HL => {
            },
            Opcode::AND_A => {
            },
            Opcode::XOR_B => {
            },
            Opcode::XOR_C => {
            },
            Opcode::XOR_D => {
            },
            Opcode::XOR_E => {
            },
            Opcode::XOR_H => {
            },
            Opcode::XOR_L => {
            },
            Opcode::XOR_HL => {
            },
            Opcode::XOR_A => {
            },
            Opcode::OR_B => {
            },
            Opcode::OR_C => {
            },
            Opcode::OR_D => {
            },
            Opcode::OR_E => {
            },
            Opcode::OR_H => {
            },
            Opcode::OR_L => {
            },
            Opcode::OR_HL => {
            },
            Opcode::OR_A => {
            },
            Opcode::CP_B => {
            },
            Opcode::CP_C => {
            },
            Opcode::CP_D => {
            },
            Opcode::CP_E => {
            },
            Opcode::CP_H => {
            },
            Opcode::CP_L => {
            },
            Opcode::CP_HL => {
            },
            Opcode::CP_A => {
            },
            Opcode::RET_NZ => {
            },
            Opcode::POP_BC => {
            },
            Opcode::JP_NZ_a16 => {
            },
            Opcode::JP_a16 => {
                self.cpu.pc = (instruction & 0xFFFF) as u16;
                skip_pc = true;
            },
            Opcode::CALL_NZ_a16 => {
            },
            Opcode::PUSH_BC => {
            },
            Opcode::ADD_A_d8 => {
            },
            Opcode::RST_00H => {
            },
            Opcode::RET_Z => {
            },
            Opcode::RET => {
            },
            Opcode::JP_Z_a16 => {
            },
            Opcode::PREFIX_CB => {
            },
            Opcode::CALL_Z_a16 => {
            },
            Opcode::CALL_a16 => {
            },
            Opcode::ADC_A_d8 => {
            },
            Opcode::RST_08H => {
            },
            Opcode::RET_NC => {
            },
            Opcode::POP_DE => {
            },
            Opcode::JP_NC_a16 => {
            },
            Opcode::CALL_NC_a16 => {
            },
            Opcode::PUSH_DE => {
            },
            Opcode::SUB_d8 => {
            },
            Opcode::RST_10H => {
            },
            Opcode::RET_C => {
            },
            Opcode::RETI => {
            },
            Opcode::JP_C_a16 => {
            },
            Opcode::CALL_C_a16 => {
            },
            Opcode::SBC_A_d8 => {
            },
            Opcode::RST_18H => {
            },
            Opcode::LDH_a8_A => {
            },
            Opcode::POP_HL => {
            },
            Opcode::LD_atC_A => {
            },
            Opcode::PUSH_HL => {
            },
            Opcode::AND_d8 => {
            },
            Opcode::RST_20H => {
            },
            Opcode::ADD_SP_r8 => {
            },
            Opcode::JP_HL => {
            },
            Opcode::LD_a16_A => {
                self.try_write((instruction & 0xFFFF) as u16, self.cpu.a)?;
            },
            Opcode::XOR_d8 => {
            },
            Opcode::RST_28H => {
            },
            Opcode::LDH_A_a8 => {
            },
            Opcode::POP_AF => {
            },
            Opcode::LD_A_atC => {
                // TODO: Have to handle memory mapping to different pieces of hardware depending on address.
                //self.cpu.a = self.ram[(0xFF00 + (self.cpu.c as u16)) as usize];
            },
            Opcode::DI => {
            },
            Opcode::PUSH_AF => {
            },
            Opcode::OR_d8 => {
            },
            Opcode::RST_30H => {
            },
            Opcode::LD_HL_SP_plus_r8 => {
            },
            Opcode::LD_SP_HL => {
            },
            Opcode::LD_A_a16 => {
                self.cpu.a = self.try_read((instruction & 0xFFFF) as u16)?;
            },
            Opcode::EI => {
            },
            Opcode::CP_d8 => {
            },
            Opcode::RST_38H => {
            },
        }
        if !skip_pc {
            self.cpu.pc += size;
        }
        Ok(base_cycles)
    }

    fn try_read(&self, address: u16) -> Result<u8, TickError> {
        match self.read(address) {
            Ok(data) => Ok(data),
            Err(error) => Err(TickError::MemoryRead { address, error }),
        }
    }

    fn try_write(&mut self, address: u16, data: u8) -> Result<(), TickError> {
        match self.write(address, data) {
            Ok(_) => Ok(()),
            Err(error) => Err(TickError::MemoryWrite { address, error }),
        }
    }

    fn read(&self, address: u16) -> Result<u8, ReadError> {
        let range = AddressRange::get(address);
        match range {
            AddressRange::ROMBank0 | AddressRange::SwitchableROMBank => {
                match &self.cartridge {
                    Some(cartridge) => Ok(cartridge.rom[range.normalize(address)]),
                    None => Err(ReadError::NoCartridge),
                }
            },
            AddressRange::VideoRAM
                => Ok(self.vram[range.normalize(address)]),
            AddressRange::SwitchableRAMBank
                => Ok(0), // TODO: Implement.
            AddressRange::InternalRAM | AddressRange::InternalRAMEcho
                => Ok(self.ram[range.normalize(address)]),
            AddressRange::SpriteAttributeMemory
                => Ok(0), // TODO: Implement.
            AddressRange::Empty
                => Err(ReadError::InvalidAddress),
            AddressRange::Io 
                => Ok(0), // TODO: Implement.
            AddressRange::InternalRAMUpper 
                => Ok(0), // TODO: Implement.
            AddressRange::InterruptEnableRegister 
                => Ok(0), // TODO: Implement.
        }
    }

    fn write(&mut self, address: u16, data: u8) -> Result<(), WriteError> {
        let range = AddressRange::get(address);
        match range {
            AddressRange::ROMBank0
                | AddressRange::SwitchableROMBank
                | AddressRange::InternalRAMEcho => Err(WriteError::ReadOnly),
            
            AddressRange::VideoRAM => {
                self.vram[range.normalize(address)] = data;
                Ok(())
            },
            AddressRange::SwitchableRAMBank => {
                // TODO: Implement.
                Ok(())
            },
            AddressRange::InternalRAM => {
                self.ram[range.normalize(address)] = data;
                Ok(())
            },
            AddressRange::SpriteAttributeMemory => {
                // TODO: Implement.
                Ok(())
            },
            AddressRange::Empty => Err(WriteError::InvalidAddress),
            AddressRange::Io => {
                // TODO: Implement.
                Ok(())
            },
            AddressRange::InternalRAMUpper => {
                // TODO: Implement.
                Ok(())
            },
            AddressRange::InterruptEnableRegister => {
                // TODO: Implement.
                Ok(())
            }
        }
    }
}

enum AddressRange {
    ROMBank0,
    SwitchableROMBank,
    VideoRAM,
    SwitchableRAMBank,
    InternalRAM,
    InternalRAMEcho,
    SpriteAttributeMemory,
    Empty,
    Io,
    InternalRAMUpper,
    InterruptEnableRegister,
}

impl AddressRange {
    fn get(address: u16) -> AddressRange {
        if address < 0x4000 {
            return AddressRange::ROMBank0;
        } else if address >= 0x4000 && address < 0x8000 {
            return AddressRange::SwitchableROMBank;
        } else if address >= 0x8000 && address < 0xA000 {
            return AddressRange::VideoRAM;
        } else if address >= 0xA000 && address < 0xC000 {
            return AddressRange::SwitchableRAMBank;
        } else if address >= 0xC000 && address < 0xE000 {
            return AddressRange::InternalRAM;
        } else if address >= 0xE000 && address < 0xFE00 {
            return AddressRange::InternalRAMEcho;
        } else if address >= 0xFE00 && address < 0xFEA0 {
            return AddressRange::SpriteAttributeMemory;
        } else if address >= 0xFEA0 && address < 0xFF00 {
            return AddressRange::Empty;
        } else if address >= 0xFF00 && address < 0xFF4C {
            return AddressRange::Io;
        } else if address >= 0xFF4C && address < 0xFF80 {
            return AddressRange::Empty;
        } else if address >= 0xFF80 && address < 0xFFFF {
            return AddressRange::InternalRAMUpper;
        } else {
            return AddressRange::InterruptEnableRegister;
        }
    }

    fn normalize(&self, address: u16) -> usize {
        let base = match self {
            AddressRange::VideoRAM => 0x8000,
            AddressRange::InternalRAM => 0xC000,
            _ => 0x0000,
        };
        return (address - base) as usize;
    }
}

#[derive(Debug)]
pub enum TickError {
    InvalidOpcode { opcode: u8, address: u16 },
    MemoryRead { address: u16, error: ReadError },
    MemoryWrite { address: u16, error: WriteError },
    NoCartridge,
}

impl TickError {
    pub fn recoverable(&self) -> bool {
        match self {
            Self::InvalidOpcode { .. }
                | Self::MemoryRead { .. }
                | Self::MemoryWrite { .. } => false,
            Self::NoCartridge => true, 
        }
    }
}

impl fmt::Display for TickError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", match self {
            Self::InvalidOpcode { opcode, address } => format!("Invalid opcode ({}) at address: {}", opcode, address),
            Self::MemoryRead { address, error } => format!("Could not read from address {:#X}: {}", address, error),
            Self::MemoryWrite { address, error } => format!("Could not write to address {:#X}: {}", address, error),
            Self::NoCartridge => "No cartridge found!".to_owned(),
        })
    }
}

#[derive(Debug)]
pub enum ReadError {
    InvalidAddress,
    NoCartridge,
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::InvalidAddress => "Invalid address",
            Self::NoCartridge => "No cartridge inserted",
        })
    }
}

#[derive(Debug)]
pub enum WriteError {
    InvalidAddress,
    ReadOnly,
}

impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::InvalidAddress => "Invalid address",
            Self::ReadOnly => "Tried to write to read-only memory",
        })
    }
}