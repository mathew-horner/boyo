use indexmap::IndexSet;

use super::opcode::Opcode;
use crate::gb::cpu::Register;
use crate::Gameboy;

pub struct Debugger {
    gameboy: Gameboy,

    // An IndexSet is used to preserve order, so "break-list" doesn't show breakpoints in an
    // arbitrary and inconsistent order.
    breakpoints: IndexSet<u16>,
}

impl Debugger {
    pub fn new(gameboy: Gameboy) -> Self {
        Self { gameboy, breakpoints: IndexSet::new() }
    }

    pub fn invoke_command(&mut self, command: &str) {
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
            Ok(Command::Continue) => {
                loop {
                    // TODO: Handle cycles.
                    if let Err(error) = self.gameboy.tick() {
                        error.realize();
                        break;
                    }
                    if self.should_break() {
                        break;
                    }
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
            Ok(Command::Next) => {
                // TODO: Clean this up.
                let pc = &self.gameboy.cpu.pc;
                match self.gameboy.try_read(*pc) {
                    Ok(opcode) => match Opcode::parse(opcode) {
                        Some(opcode) => {
                            match self.gameboy.try_cartridge_read_bytes(*pc, opcode.size() as u16) {
                                Ok(instruction) => {
                                    print(format!("{pc:#X}: {instruction:#08X}"));
                                },
                                Err(error) => {
                                    print(format!("Unable to read instruction: {error}",));
                                },
                            }
                        },
                        None => print(format!("Unable to parse opcode: {opcode:#04X}",)),
                    },
                    Err(error) => print(format!("Unable to read opcode: {error}",)),
                }
            },
            Ok(Command::Registers) => {
                print_many(
                    self.gameboy
                        .cpu
                        .registers()
                        .map(|Register { name, value }| format!("{name}: {value:#X}")),
                );
            },
            Ok(Command::Step) => {
                match self.gameboy.tick() {
                    // We don't need to track the cycle count because we will draw the frame after
                    // every instruction anyway.
                    Ok(_) => {},
                    Err(error) => error.realize(),
                }
            },
            Err(error) => {
                print(error.to_string());
            },
        }
    }

    fn should_break(&self) -> bool {
        self.breakpoints.contains(&self.gameboy.cpu.pc)
    }
}

enum Command {
    BreakAdd(u16),
    BreakList,
    BreakRemove(u16),
    Continue,
    Exit,
    Help,
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
            "next" if tokens.len() == 1 => Ok(Command::Next),
            "registers" if tokens.len() == 1 => Ok(Command::Registers),
            "step" if tokens.len() == 1 => Ok(Command::Step),

            // Valid commands should be enumerated here as a fall-through case in scenarios where an
            // invalid number of tokens are provided.
            "break-add" | "break-remove" | "break-list" | "continue" | "exit" | "help" | "next"
            | "registers" | "step" => Err(CommandParseError::InvalidFormat),

            other => Err(CommandParseError::InvalidCommand(other)),
        }
    }
}

fn parse_hex_address(address: &str) -> Result<u16, CommandParseError<'_>> {
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
