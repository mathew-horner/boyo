use std::collections::{vec_deque, VecDeque};
use std::fmt::{UpperHex, Write as _};
use std::io::{self, Write as _};
use std::thread::sleep;
use std::time::{Duration, Instant};

use console::Term;
use indexmap::IndexSet;

const INITIAL_PC: u16 = 0x0100;
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

    fn cycle(&mut self) {
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
    fn peek_instruction_state(&self) -> Result<InstructionState, u8> {
        let mut state = self.instruction_state.clone();
        if state.is_done() {
            let byte = self.system.rom[self.system.pc as usize];
            let instruction = Instruction::from_opcode(byte).ok_or(byte)?;
            state = InstructionState { instruction, m_cycle: 0 };
        }
        state.m_cycle += 1;
        Ok(state)
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

    fn registers<'a>(&'a self) -> Registers<'a> {
        Registers { system: self, idx: 0 }
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

struct Registers<'a> {
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

struct Register {
    name: &'static str,
    value: RegisterValue,
}

enum RegisterValue {
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

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
enum Instruction {
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

    /// `JP nn`
    ///
    /// Unconditional jump to the absolute address specified by the 16-bit
    /// immediate operand nn.
    JP_nn { address: u16 },
}

impl Instruction {
    fn from_opcode(opcode: u8) -> Option<Self> {
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
            0x36 => None,
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

    fn cycles(&self) -> usize {
        match self {
            Self::Initial => 0,
            Self::NOP | Self::LD_r_r { .. } => 1,
            Self::LD_r_n { .. } | Self::LD_r_HL { .. } | Self::LD_HL_r { .. } => 2,
            Self::JP_nn { .. } => 4,
        }
    }
}

pub struct Debugger {
    gameboy: Gameboy,
    command_history: CommandHistory,

    // An IndexSet is used to preserve order, so "break-list" doesn't show breakpoints in an
    // arbitrary and inconsistent order.
    breakpoints: IndexSet<u16>,
}

impl Debugger {
    pub fn new(gameboy: Gameboy) -> Self {
        Self { gameboy, command_history: CommandHistory::new(10), breakpoints: IndexSet::new() }
    }

    fn invoke_command(&mut self, command: &str) {
        match Command::parse(command) {
            Ok(Command::BreakAdd(address)) => {
                self.breakpoints.insert(address);
                print(format!("breakpoint added @ {address:#X}"));
            },
            Ok(Command::BreakRemove(address)) => {
                self.breakpoints.retain(|bp| *bp != address);
                print(format!("breakpoint(s) removed @ {address:#X}"));
            },
            Ok(Command::BreakList) => {
                print_many(self.breakpoints.iter().map(|bp| format!("{bp:#X}")));
            },
            Ok(Command::Continue) => loop {
                self.gameboy.cycle();
                if self.should_break() {
                    break;
                }
            },
            Ok(Command::Exit) => std::process::exit(0),
            Ok(Command::Help) => {
                // This strange syntax is to make it so the raw string literal prints without a
                // leading empty line (hence the [1..]) at the end.
                print(
                    r#"
-------------
Boyo Debugger
-------------
Commands
* break-add <address> - Adds a new breakpoint at the given (hex) address.
* break-list - Shows all the currently active breakpoints.
* break-remove <address> - Removes an existing breakpoint at the given (hex) address, if it exists.
* continue - Begins execution until a breakpoint is hit.
* exit - Exits the program.
* help - How you got here.
* next - Displays the next instruction to be executed.
* registers - Displays the contents of all cpu registers.
* step - Executes a single instruction.
"#,
                );
            },
            Ok(Command::History) => {
                print_many(self.command_history.iter());
            },
            Ok(Command::Next) => {
                self.print_next_instruction("Next");
            },
            Ok(Command::Registers) => {
                print_many(
                    self.gameboy
                        .system
                        .registers()
                        .map(|Register { name, value }| format!("{name}: {value:#X}")),
                );
            },
            Ok(Command::Step) => {
                self.print_next_instruction("Exec");
                self.gameboy.cycle();
                self.print_next_instruction("Next");
            },
            Err(error) => {
                print(error.to_string());
            },
        }
        self.command_history.push(command.to_owned());
    }

    fn print_next_instruction(&self, label: &str) {
        let mut message = format!("{label}: ");
        match self.gameboy.peek_instruction_state() {
            Ok(state) => write!(&mut message, "{state:?}").unwrap(),
            Err(opcode) => write!(&mut message, "0x{opcode:02X}").unwrap(),
        };
        print(message)
    }

    fn should_break(&self) -> bool {
        self.breakpoints.contains(&self.gameboy.system.pc)
    }
}

pub fn run_terminal_debugger(mut debugger: Debugger) -> ! {
    debugger.print_next_instruction("Next");
    let term = Term::stdout();
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        let mut command = String::new();

        // history_idx = 0 refers to the command buffer. history_idx = 1.. refers to
        // history @ history_idx - 1.
        let mut history_idx: usize = 0;

        loop {
            match term.read_key().unwrap() {
                console::Key::Enter => {
                    println!();
                    break;
                },
                console::Key::Backspace => {
                    let _ = command.pop();
                    term.clear_chars(1).unwrap();
                },
                console::Key::ArrowUp if history_idx < debugger.command_history.len() => {
                    history_idx += 1;
                    update_command_display(&debugger, &term, history_idx, &command);
                },
                console::Key::ArrowDown if history_idx > 0 => {
                    history_idx -= 1;
                    update_command_display(&debugger, &term, history_idx, &command);
                },
                console::Key::Char(char) => {
                    print!("{char}");
                    io::stdout().flush().unwrap();
                    command.push(char);
                },
                _ => {},
            };
        }

        // TODO: Make it so we don't have to clone here...
        let command = match history_idx {
            0 => command.trim().to_owned(),
            idx => debugger.command_history.entry(idx - 1).unwrap().to_owned(),
        };

        debugger.invoke_command(&command);
    }
}

fn update_command_display(debugger: &Debugger, term: &Term, history_idx: usize, command: &str) {
    term.clear_line().unwrap();

    let value = match history_idx {
        0 => command,
        // We never let history_idx outside the bounds of the history, so this
        // unwrap is safe.
        idx => debugger.command_history.entry(idx - 1).unwrap(),
    };

    print!("> {value}");
    io::stdout().flush().unwrap();
}

#[derive(Debug, Eq, PartialEq)]
enum Command {
    BreakAdd(u16),
    BreakList,
    BreakRemove(u16),
    Continue,
    Exit,
    Help,
    History,
    Next,
    Registers,
    Step,
}

#[derive(Debug, thiserror::Error)]
enum CommandParseError<'a> {
    #[error("invalid command: {0}")]
    InvalidCommand(&'a str),
    #[error("invalid format")]
    InvalidFormat,
    #[error("invalid breakpoint address")]
    InvalidBreakpointAddress,
}

impl Command {
    fn parse<'a>(s: &'a str) -> Result<Self, CommandParseError<'a>> {
        let tokens: Vec<_> = s.split(" ").collect();
        match tokens[0] {
            "break-add" if tokens.len() == 2 => {
                let address = parse_hex_address(&tokens[1])?;
                Ok(Command::BreakAdd(address))
            },
            "break-remove" if tokens.len() == 2 => {
                let address = parse_hex_address(&tokens[1])?;
                Ok(Command::BreakRemove(address))
            },
            "break-list" if tokens.len() == 1 => Ok(Command::BreakList),
            "continue" if tokens.len() == 1 => Ok(Command::Continue),
            "exit" if tokens.len() == 1 => Ok(Command::Exit),
            "help" if tokens.len() == 1 => Ok(Command::Help),
            "history" if tokens.len() == 1 => Ok(Command::History),
            "next" if tokens.len() == 1 => Ok(Command::Next),
            "registers" if tokens.len() == 1 => Ok(Command::Registers),
            "step" if tokens.len() == 1 => Ok(Command::Step),

            // Valid commands should be enumerated here as a fall-through case in scenarios where an
            // invalid number of tokens are provided.
            "break-add" | "break-remove" | "break-list" | "continue" | "exit" | "help"
            | "history" | "next" | "registers" | "step" => Err(CommandParseError::InvalidFormat),

            other => Err(CommandParseError::InvalidCommand(other)),
        }
    }
}

fn parse_hex_address(address: &str) -> Result<u16, CommandParseError<'_>> {
    let address = address.strip_prefix("0x").unwrap_or(address);
    u16::from_str_radix(address, 16).map_err(|_| CommandParseError::InvalidBreakpointAddress)
}

fn print(message: impl AsRef<str>) {
    print_many(std::iter::once(message));
}

fn print_many<I, S>(messages: I)
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    for message in messages {
        println!("{}", message.as_ref().trim());
    }
}
struct CommandHistory {
    queue: VecDeque<String>,
    size: usize,
}

impl CommandHistory {
    fn new(size: usize) -> Self {
        Self { queue: VecDeque::with_capacity(size), size }
    }

    fn push(&mut self, value: impl Into<String>) {
        let value = value.into();
        if value == "history" || self.queue.back().map(|back| back == &value).unwrap_or(false) {
            return;
        }

        // Pop first so we only ever need to have space for N items allocated.
        if self.queue.len() == self.size {
            self.queue.pop_front();
        }

        self.queue.push_back(value);
    }

    fn iter<'a>(&'a self) -> vec_deque::Iter<'a, String> {
        self.queue.iter()
    }

    fn entry<'a>(&'a self, idx: usize) -> Option<&'a str> {
        self.queue.get(self.len() - 1 - idx).map(String::as_str)
    }

    fn len(&self) -> usize {
        self.queue.len()
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

    #[test]
    fn parse_breakpoint_commands() {
        assert_eq!(Command::parse("break-add 0xFFFF").unwrap(), Command::BreakAdd(0xFFFF));
        assert_eq!(Command::parse("break-add FFFF").unwrap(), Command::BreakAdd(0xFFFF));
        assert_eq!(Command::parse("break-add 0x0").unwrap(), Command::BreakAdd(0));
        assert_eq!(Command::parse("break-add 0").unwrap(), Command::BreakAdd(0));

        assert_eq!(Command::parse("break-remove 0xFFFF").unwrap(), Command::BreakRemove(0xFFFF));
        assert_eq!(Command::parse("break-remove FFFF").unwrap(), Command::BreakRemove(0xFFFF));
        assert_eq!(Command::parse("break-remove 0x0").unwrap(), Command::BreakRemove(0));
        assert_eq!(Command::parse("break-remove 0").unwrap(), Command::BreakRemove(0));
    }

    #[test]
    fn command_history() {
        const SIZE: usize = 5;

        let mut history = CommandHistory::new(SIZE);
        for i in 0..SIZE + 1 {
            history.push(format!("command-{}", i + 1));
        }

        // "history" should not be pushed
        history.push("history");

        // Repeat value should not be pushed
        history.push("command-6");

        let values: Vec<_> = history.iter().map(String::as_str).collect();
        assert_eq!(&values, &["command-2", "command-3", "command-4", "command-5", "command-6"]);
    }
}
