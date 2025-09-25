use std::io::{self, Write as _};

use console::Term;
use indexmap::IndexSet;

use crate::command_history::CommandHistory;
use crate::gb::{Gameboy, Register};

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
                println!("breakpoint added @ {address:#X}");
            },
            Ok(Command::BreakRemove(address)) => {
                self.breakpoints.retain(|bp| *bp != address);
                println!("breakpoint(s) removed @ {address:#X}");
            },
            Ok(Command::BreakList) => {
                self.breakpoints.iter().for_each(|bp| println!("{bp:#X}"));
            },
            Ok(Command::Continue) => loop {
                self.gameboy.cycle();
                if self.should_break() {
                    break;
                }
            },
            Ok(Command::ContinueUntilNotImpl) => loop {
                match self.gameboy.peek_instruction_state() {
                    Ok(_) => self.gameboy.cycle(),
                    Err(opcode) => {
                        println!("not impl: 0x{opcode:02X}");
                        break;
                    },
                };
            },
            Ok(Command::Exit) => std::process::exit(0),
            Ok(Command::Help) => {
                #[rustfmt::skip]
                println!(
r#"-------------
Boyo Debugger
-------------
Commands
* break-add <address> - Adds a new breakpoint at the given (hex) address.
* break-list - Shows all the currently active breakpoints.
* break-remove <address> - Removes an existing breakpoint at the given (hex) address, if it exists.
* continue - Begins execution until a breakpoint is hit.
* continue-until-not-impl - Begins execution until a non-implemented opcode is encountered.
* exit - Exits the program.
* help - How you got here.
* next - Displays the next instruction to be executed.
* registers - Displays the contents of all cpu registers.
* step - Executes a single instruction."#
                );
            },
            Ok(Command::History) => {
                self.command_history.iter().for_each(|entry| println!("{entry}"));
            },
            Ok(Command::Next) => {
                self.print_next_instruction();
            },
            Ok(Command::Registers) => {
                self.gameboy
                    .registers()
                    .for_each(|Register { name, value }| println!("{name}: {value:#X}"));
            },
            Ok(Command::Step) => {
                self.print_next_instruction();
                self.gameboy.cycle();
            },
            Err(error) => {
                eprintln!("{error}");
            },
        }
        self.command_history.push(command.to_owned());
    }

    fn print_next_instruction(&self) {
        match self.gameboy.peek_instruction_state() {
            Ok(state) => println!("{state:?}"),
            Err(opcode) => println!("0x{opcode:02X}"),
        };
    }

    fn should_break(&self) -> bool {
        self.breakpoints.contains(&self.gameboy.pc())
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
    ContinueUntilNotImpl,
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
            "continue-until-not-impl" if tokens.len() == 1 => Ok(Command::ContinueUntilNotImpl),
            "exit" if tokens.len() == 1 => Ok(Command::Exit),
            "help" if tokens.len() == 1 => Ok(Command::Help),
            "history" if tokens.len() == 1 => Ok(Command::History),
            "next" if tokens.len() == 1 => Ok(Command::Next),
            "registers" if tokens.len() == 1 => Ok(Command::Registers),
            "step" if tokens.len() == 1 => Ok(Command::Step),

            // Valid commands should be enumerated here as a fall-through case in scenarios where an
            // invalid number of tokens are provided.
            "break-add"
            | "break-remove"
            | "break-list"
            | "continue"
            | "continue-until-not-impl"
            | "exit"
            | "help"
            | "history"
            | "next"
            | "registers"
            | "step" => Err(CommandParseError::InvalidFormat),

            other => Err(CommandParseError::InvalidCommand(other)),
        }
    }
}

fn parse_hex_address(address: &str) -> Result<u16, CommandParseError<'_>> {
    let address = address.strip_prefix("0x").unwrap_or(address);
    u16::from_str_radix(address, 16).map_err(|_| CommandParseError::InvalidBreakpointAddress)
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
}
