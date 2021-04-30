mod cartridge;
mod cpu;
mod debugger;
mod opcode;
pub use cartridge::Cartridge;
pub use debugger::Debugger;
use cartridge::CartridgeReadError;
use cpu::LR35902;
use opcode::{ Opcode, OpcodeType };
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
        let opcode = self.try_cartridge_read_bytes(self.cpu.pc, 1)? as u8;
        let opcode = match Opcode::parse(opcode) {
            Some(op) => op,
            None => {
                return Err(TickError::InvalidOpcode { opcode, address: self.cpu.pc });
            }
        };
        // TODO-PERF: avoid re-reading memory here, but it's the simplest solution atm.
        let instruction = self.try_cartridge_read_bytes(self.cpu.pc, opcode.size() as u16)?;
        if !self.execute(&opcode, instruction as u16)? {
            self.cpu.pc += opcode.size() as u16;
        }
        Ok(opcode.base_cycles())
    }

    // TODO-CQ: Find a way to combine opcode & instruction for brevity?
    fn execute(&mut self, opcode: &Opcode, instruction: u16) -> Result<bool, TickError> {
        let mut skip_pc = false;
        match opcode.type_ {
            OpcodeType::NOP => (),
            OpcodeType::LD_BC_d16 => {
                self.cpu.set_bc((instruction & 0xFFFF) as u16);
            },
            OpcodeType::LD_BC_A => {
                self.try_write(self.cpu.bc(), self.cpu.a)?;
            },
            OpcodeType::INC_BC => {
            },
            OpcodeType::INC_B => {
            },
            OpcodeType::DEC_B => {
            },
            OpcodeType::LD_B_d8 => {
                self.cpu.b = (instruction & 0xFF) as u8;
            },
            OpcodeType::RLCA => {
                // TODO-Q: Is this the right thing to do? How do we set Z?
                self.cpu.set_flags(1, 0, 0, self.cpu.z());
            },
            OpcodeType::LD_a16_SP => {
            },
            OpcodeType::ADD_HL_BC => {
            },
            OpcodeType::LD_A_BC => {
                self.cpu.a = self.try_read(self.cpu.bc())?;
            },
            OpcodeType::DEC_BC => {
            },
            OpcodeType::INC_C => {
            },
            OpcodeType::DEC_C => {
            },
            OpcodeType::LD_C_d8 => {
                self.cpu.c = (instruction & 0xFF) as u8;
            },
            OpcodeType::RRCA => {
            },
            OpcodeType::STOP_0 => {
            },
            OpcodeType::LD_DE_d16 => {
            },
            OpcodeType::LD_DE_A => {
                self.try_write(self.cpu.de(), self.cpu.a)?;
            },
            OpcodeType::INC_DE => {
            },
            OpcodeType::INC_D => {
            },
            OpcodeType::DEC_D => {
            },
            OpcodeType::LD_D_d8 => {
                self.cpu.d = (instruction & 0xFF) as u8;
            },
            OpcodeType::RLA => {
            },
            OpcodeType::JR_r8 => {
            },
            OpcodeType::ADD_HL_DE => {
            },
            OpcodeType::LD_A_DE => {
                self.cpu.a = self.try_read(self.cpu.de())?;
            },
            OpcodeType::DEC_DE => {
            },
            OpcodeType::INC_E => {
            },
            OpcodeType::DEC_E => {
            },
            OpcodeType::LD_E_d8 => {
                self.cpu.e = (instruction & 0xFF) as u8;
            },
            OpcodeType::RRA => {
            },
            OpcodeType::JR_NZ_r8 => {
            },
            OpcodeType::LD_HL_d16 => {
            },
            OpcodeType::LD_HL_plus_A => {
            },
            OpcodeType::INC_HL => {
            },
            OpcodeType::INC_H => {
            },
            OpcodeType::DEC_H => {
            },
            OpcodeType::LD_H_d8 => {
                self.cpu.h = (instruction & 0xFF) as u8;
            },
            OpcodeType::DAA => {
            },
            OpcodeType::JR_Z_r8 => {
            },
            OpcodeType::ADD_HL_HL => {
            },
            OpcodeType::LD_A_HL_ => {
            },
            OpcodeType::DEC_HL => {
            },
            OpcodeType::INC_L => {
            },
            OpcodeType::DEC_L => {
            },
            OpcodeType::LD_L_d8 => {
                self.cpu.l = (instruction & 0xFF) as u8;
            },
            OpcodeType::CPL => {
            },
            OpcodeType::JR_NC_r8 => {
            },
            OpcodeType::LD_SP_d16 => {
            },
            OpcodeType::LD_HL_minus_A => {
            },
            OpcodeType::INC_SP => {
            },
            OpcodeType::INC_atHL => {
            },
            OpcodeType::DEC_atHL => {
            },
            OpcodeType::LD_HL_d8 => {
            },
            OpcodeType::SCF => {
            },
            OpcodeType::JR_C_r8 => {
            },
            OpcodeType::ADD_HL_SP => {
            },
            OpcodeType::LD_A_HL_minus => {
            },
            OpcodeType::DEC_SP => {
            },
            OpcodeType::INC_A => {
            },
            OpcodeType::DEC_A => {
            },
            OpcodeType::LD_A_d8 => {
            },
            OpcodeType::CCF => {
            },
            OpcodeType::LD_B_B => (),
            OpcodeType::LD_B_C => {
                self.cpu.b = self.cpu.c;
            },
            OpcodeType::LD_B_D => {
                self.cpu.b = self.cpu.d;
            },
            OpcodeType::LD_B_E => {
                self.cpu.b = self.cpu.e;
            },
            OpcodeType::LD_B_H => {
                self.cpu.b = self.cpu.h;
            },
            OpcodeType::LD_B_L => {
                self.cpu.b = self.cpu.l;
            },
            OpcodeType::LD_B_HL => {
                self.cpu.b = self.try_read(self.cpu.hl())?;
            },
            OpcodeType::LD_B_A => {
                self.cpu.b = self.cpu.a;
            },
            OpcodeType::LD_C_B => {
            },
            OpcodeType::LD_C_C => {
            },
            OpcodeType::LD_C_D => {
            },
            OpcodeType::LD_C_E => {
            },
            OpcodeType::LD_C_H => {
            },
            OpcodeType::LD_C_L => {
            },
            OpcodeType::LD_C_HL => {
            },
            OpcodeType::LD_C_A => {
                self.cpu.c = self.cpu.a;
            },
            OpcodeType::LD_D_B => {
            },
            OpcodeType::LD_D_C => {
            },
            OpcodeType::LD_D_D => {
            },
            OpcodeType::LD_D_E => {
            },
            OpcodeType::LD_D_H => {
            },
            OpcodeType::LD_D_L => {
            },
            OpcodeType::LD_D_HL => {
            },
            OpcodeType::LD_D_A => {
                self.cpu.d = self.cpu.a;
            },
            OpcodeType::LD_E_B => {
            },
            OpcodeType::LD_E_C => {
            },
            OpcodeType::LD_E_D => {
            },
            OpcodeType::LD_E_E => {
            },
            OpcodeType::LD_E_H => {
            },
            OpcodeType::LD_E_L => {
            },
            OpcodeType::LD_E_HL => {
            },
            OpcodeType::LD_E_A => {
                self.cpu.e = self.cpu.a;
            },
            OpcodeType::LD_H_B => {
            },
            OpcodeType::LD_H_C => {
            },
            OpcodeType::LD_H_D => {
            },
            OpcodeType::LD_H_E => {
            },
            OpcodeType::LD_H_H => {
            },
            OpcodeType::LD_H_L => {
            },
            OpcodeType::LD_H_HL => {
            },
            OpcodeType::LD_H_A => {
                self.cpu.h = self.cpu.a;
            },
            OpcodeType::LD_L_B => {
            },
            OpcodeType::LD_L_C => {
            },
            OpcodeType::LD_L_D => {
            },
            OpcodeType::LD_L_E => {
            },
            OpcodeType::LD_L_H => {
            },
            OpcodeType::LD_L_L => {
            },
            OpcodeType::LD_L_HL => {
            },
            OpcodeType::LD_L_A => {
                self.cpu.l = self.cpu.a;
            },
            OpcodeType::LD_HL_B => {
                self.try_write(self.cpu.hl(), self.cpu.b)?;
            },
            OpcodeType::LD_HL_C => {
            },
            OpcodeType::LD_HL_D => {
            },
            OpcodeType::LD_HL_E => {
            },
            OpcodeType::LD_HL_H => {
            },
            OpcodeType::LD_HL_L => {
            },
            OpcodeType::HALT => {
            },
            OpcodeType::LD_HL_A => {
                self.try_write(self.cpu.hl(), self.cpu.a)?;
            },
            OpcodeType::LD_A_B => {
                self.cpu.a = self.cpu.b;
            },
            OpcodeType::LD_A_C => {
                self.cpu.a = self.cpu.c;
            },
            OpcodeType::LD_A_D => {
                self.cpu.a = self.cpu.d;
            },
            OpcodeType::LD_A_E => {
                self.cpu.a = self.cpu.e;
            },
            OpcodeType::LD_A_H => {
                self.cpu.a = self.cpu.h;
            },
            OpcodeType::LD_A_L => {
                self.cpu.a = self.cpu.l;
            },
            OpcodeType::LD_A_HL => {
                self.cpu.a = self.try_read(self.cpu.hl())?;
            },
            OpcodeType::LD_A_A => (),
            OpcodeType::ADD_A_B => {
            },
            OpcodeType::ADD_A_C => {
            },
            OpcodeType::ADD_A_D => {
            },
            OpcodeType::ADD_A_E => {
            },
            OpcodeType::ADD_A_H => {
            },
            OpcodeType::ADD_A_L => {
            },
            OpcodeType::ADD_A_HL => {
            },
            OpcodeType::ADD_A_A => {
            },
            OpcodeType::ADC_A_B => {
            },
            OpcodeType::ADC_A_C => {
            },
            OpcodeType::ADC_A_D => {
            },
            OpcodeType::ADC_A_E => {
            },
            OpcodeType::ADC_A_H => {
            },
            OpcodeType::ADC_A_L => {
            },
            OpcodeType::ADC_A_HL => {
            },
            OpcodeType::ADC_A_A => {
            },
            OpcodeType::SUB_B => {
            },
            OpcodeType::SUB_C => {
            },
            OpcodeType::SUB_D => {
            },
            OpcodeType::SUB_E => {
            },
            OpcodeType::SUB_H => {
            },
            OpcodeType::SUB_L => {
            },
            OpcodeType::SUB_HL => {
            },
            OpcodeType::SUB_A => {
            },
            OpcodeType::SBC_A_B => {
            },
            OpcodeType::SBC_A_C => {
            },
            OpcodeType::SBC_A_D => {
            },
            OpcodeType::SBC_A_E => {
            },
            OpcodeType::SBC_A_H => {
            },
            OpcodeType::SBC_A_L => {
            },
            OpcodeType::SBC_A_HL => {
            },
            OpcodeType::SBC_A_A => {
            },
            OpcodeType::AND_B => {
            },
            OpcodeType::AND_C => {
            },
            OpcodeType::AND_D => {
            },
            OpcodeType::AND_E => {
            },
            OpcodeType::AND_H => {
            },
            OpcodeType::AND_L => {
            },
            OpcodeType::AND_HL => {
            },
            OpcodeType::AND_A => {
            },
            OpcodeType::XOR_B => {
            },
            OpcodeType::XOR_C => {
            },
            OpcodeType::XOR_D => {
            },
            OpcodeType::XOR_E => {
            },
            OpcodeType::XOR_H => {
            },
            OpcodeType::XOR_L => {
            },
            OpcodeType::XOR_HL => {
            },
            OpcodeType::XOR_A => {
            },
            OpcodeType::OR_B => {
            },
            OpcodeType::OR_C => {
            },
            OpcodeType::OR_D => {
            },
            OpcodeType::OR_E => {
            },
            OpcodeType::OR_H => {
            },
            OpcodeType::OR_L => {
            },
            OpcodeType::OR_HL => {
            },
            OpcodeType::OR_A => {
            },
            OpcodeType::CP_B => {
            },
            OpcodeType::CP_C => {
            },
            OpcodeType::CP_D => {
            },
            OpcodeType::CP_E => {
            },
            OpcodeType::CP_H => {
            },
            OpcodeType::CP_L => {
            },
            OpcodeType::CP_HL => {
            },
            OpcodeType::CP_A => {
            },
            OpcodeType::RET_NZ => {
            },
            OpcodeType::POP_BC => {
            },
            OpcodeType::JP_NZ_a16 => {
            },
            OpcodeType::JP_a16 => {
                self.cpu.pc = (instruction & 0xFFFF) as u16;
                skip_pc = true;
            },
            OpcodeType::CALL_NZ_a16 => {
            },
            OpcodeType::PUSH_BC => {
            },
            OpcodeType::ADD_A_d8 => {
            },
            OpcodeType::RST_00H => {
            },
            OpcodeType::RET_Z => {
            },
            OpcodeType::RET => {
            },
            OpcodeType::JP_Z_a16 => {
            },
            OpcodeType::PREFIX_CB => {
            },
            OpcodeType::CALL_Z_a16 => {
            },
            OpcodeType::CALL_a16 => {
            },
            OpcodeType::ADC_A_d8 => {
            },
            OpcodeType::RST_08H => {
            },
            OpcodeType::RET_NC => {
            },
            OpcodeType::POP_DE => {
            },
            OpcodeType::JP_NC_a16 => {
            },
            OpcodeType::CALL_NC_a16 => {
            },
            OpcodeType::PUSH_DE => {
            },
            OpcodeType::SUB_d8 => {
            },
            OpcodeType::RST_10H => {
            },
            OpcodeType::RET_C => {
            },
            OpcodeType::RETI => {
            },
            OpcodeType::JP_C_a16 => {
            },
            OpcodeType::CALL_C_a16 => {
            },
            OpcodeType::SBC_A_d8 => {
            },
            OpcodeType::RST_18H => {
            },
            OpcodeType::LDH_a8_A => {
            },
            OpcodeType::POP_HL => {
            },
            OpcodeType::LD_atC_A => {
            },
            OpcodeType::PUSH_HL => {
            },
            OpcodeType::AND_d8 => {
            },
            OpcodeType::RST_20H => {
            },
            OpcodeType::ADD_SP_r8 => {
            },
            OpcodeType::JP_HL => {
            },
            OpcodeType::LD_a16_A => {
                self.try_write((instruction & 0xFFFF) as u16, self.cpu.a)?;
            },
            OpcodeType::XOR_d8 => {
            },
            OpcodeType::RST_28H => {
            },
            OpcodeType::LDH_A_a8 => {
            },
            OpcodeType::POP_AF => {
            },
            OpcodeType::LD_A_atC => {
                // TODO: Have to handle memory mapping to different pieces of hardware depending on address.
                //self.cpu.a = self.ram[(0xFF00 + (self.cpu.c as u16)) as usize];
            },
            OpcodeType::DI => {
            },
            OpcodeType::PUSH_AF => {
            },
            OpcodeType::OR_d8 => {
            },
            OpcodeType::RST_30H => {
            },
            OpcodeType::LD_HL_SP_plus_r8 => {
            },
            OpcodeType::LD_SP_HL => {
            },
            OpcodeType::LD_A_a16 => {
                self.cpu.a = self.try_read((instruction & 0xFFFF) as u16)?;
            },
            OpcodeType::EI => {
            },
            OpcodeType::CP_d8 => {
            },
            OpcodeType::RST_38H => {
            },
        }
        Ok(skip_pc)
    }

    fn try_cartridge_read_bytes(&self, address: u16, count: u16) -> Result<u32, TickError> {
        // TODO-PERF: check if as_ref() has a large amount of overhead?
        let cartridge = self.cartridge.as_ref().unwrap();
        match cartridge.read_bytes(address, count) {
            Ok(value) => Ok(value),
            Err(error) => Err(TickError::CartridgeRead { address, error })
        }
    }

    pub fn try_read(&self, address: u16) -> Result<u8, TickError> {
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
    CartridgeRead { address: u16, error: CartridgeReadError },
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
                | Self::MemoryWrite { .. }
                | Self::CartridgeRead { .. } => false,
            Self::NoCartridge => true, 
        }
    }

    pub fn realize(&self) {
        println!("{}", self);
        if !self.recoverable() {
            std::process::exit(0);
        }
    }
}

impl fmt::Display for TickError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", match self {
            Self::CartridgeRead { address, error } => format!("Could not read from cartridge at address {:#X}: {}", address, error),
            Self::InvalidOpcode { opcode, address } => format!("Invalid opcode ({}) at address: {}", opcode, address),
            Self::MemoryRead { address, error } => format!("Could not read from memory at address {:#X}: {}", address, error),
            Self::MemoryWrite { address, error } => format!("Could not write to memory at address {:#X}: {}", address, error),
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
