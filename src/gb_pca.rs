use std::collections::{vec_deque, VecDeque};
use std::io::{self, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

use console::Term;
use indexmap::IndexSet;

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
    #[cfg(test)]
    fn no_cartridge() -> Self {
        Self::new(Vec::new())
    }

    pub fn new(rom: Vec<u8>) -> Self {
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
            rom,
            instruction_state: InstructionState::default(),
        }
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

    fn registers<'a>(&'a self) -> Registers<'a> {
        Registers { gb: self, idx: 0 }
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

struct Registers<'a> {
    gb: &'a Gameboy,
    idx: usize,
}

impl<'a> Iterator for Registers<'a> {
    type Item = Register;

    fn next(&mut self) -> Option<Self::Item> {
        let register = match self.idx {
            0 => Register { name: "a", value: self.gb.a },
            1 => Register { name: "b", value: self.gb.b },
            2 => Register { name: "c", value: self.gb.c },
            3 => Register { name: "d", value: self.gb.d },
            4 => Register { name: "e", value: self.gb.e },
            5 => Register { name: "f", value: self.gb.f },
            6 => Register { name: "h", value: self.gb.h },
            7 => Register { name: "l", value: self.gb.l },
            _ => return None,
        };
        self.idx += 1;
        Some(register)
    }
}

struct Register {
    name: &'static str,
    value: u8,
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
                todo!("next command was easy to implement with the non cycle based emulator, but will take some work here");
            },
            Ok(Command::Registers) => {
                print_many(
                    self.gameboy
                        .registers()
                        .map(|Register { name, value }| format!("{name}: {value:#X}")),
                );
            },
            Ok(Command::Step) => {
                self.gameboy.cycle();
            },
            Err(error) => {
                print(error.to_string());
            },
        }
        self.command_history.push(command.to_owned());
    }

    fn should_break(&self) -> bool {
        self.breakpoints.contains(&self.gameboy.pc)
    }

    fn history_entry<'a>(&'a self, idx: usize) -> Option<&'a str> {
        self.command_history.queue.get(self.history_len() - 1 - idx).map(String::as_str)
    }

    fn history_len(&self) -> usize {
        self.command_history.queue.len()
    }
}

pub fn run_terminal_debugger(mut debugger: Debugger) -> ! {
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
                console::Key::ArrowUp if history_idx < debugger.history_len() => {
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
            idx => debugger.history_entry(idx - 1).unwrap().to_owned(),
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
        idx => debugger.history_entry(idx - 1).unwrap(),
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
    println!();
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn register16_combines_8bit_registers() {
        let mut gb = Gameboy::no_cartridge();

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
