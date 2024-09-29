use std::collections::vec_deque::{self, VecDeque};

use indexmap::IndexSet;

use crate::gb::cpu::Register;
use crate::gb::opcode::Opcode;
use crate::Gameboy;

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
            Ok(Command::History) => {
                print_many(self.command_history.iter());
            },
            Ok(Command::Next) => {
                // TODO: Clean this up.
                let pc = &self.gameboy.cpu.pc;
                match self.gameboy.try_read(*pc) {
                    Ok(opcode) => match Opcode::parse(opcode) {
                        Some(opcode) => {
                            match self.gameboy.try_cartridge_read_bytes(*pc, opcode.size() as u16) {
                                Ok(instruction) => {
                                    print_many(
                                        [
                                            format!("{opcode:?}"),
                                            format!("{pc:#X}: {instruction:#08X}"),
                                        ]
                                        .iter(),
                                    );
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
        self.command_history.push(command.to_owned());
    }

    fn should_break(&self) -> bool {
        self.breakpoints.contains(&self.gameboy.cpu.pc)
    }

    pub fn history_entry<'a>(&'a self, idx: usize) -> Option<&'a str> {
        self.command_history.queue.get(self.history_len() - 1 - idx).map(String::as_str)
    }

    pub fn history_len(&self) -> usize {
        self.command_history.queue.len()
    }
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
