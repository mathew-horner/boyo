use super::opcode::Opcode;
use crate::gb::cpu::Register;
use crate::Gameboy;

struct Breakpoint {
    address: u16,
}

pub struct Debugger {
    gameboy: Gameboy,
    breakpoints: Vec<Breakpoint>,
}

impl Debugger {
    pub fn new(gameboy: Gameboy) -> Self {
        Self { gameboy, breakpoints: Vec::new() }
    }

    pub fn invoke_command(&mut self, command: &str) {
        match Command::parse(command) {
            Ok(Command::BreakAdd(address)) => {
                self.breakpoints.push(Breakpoint { address });
                println!("breakpoint added @ {address:#X}");
            },
            Ok(Command::BreakRemove(address)) => {
                self.breakpoints.retain(|breakpoint| breakpoint.address != address);
                println!("breakpoint(s) removed @ {address:#X}");
            },
            Ok(Command::BreakList) => {
                for breakpoint in &self.breakpoints {
                    println!("{:#X}", breakpoint.address);
                }
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
                // TODO: Derive usage strings and descriptions from Command.
                println!("---------------");
                println!(" Boyo Debugger");
                println!("---------------\n");
                println!("Commands");
                println!(
                    "* break-add <address> - Adds a new breakpoint at the given (hex) address."
                );
                println!("* break-list - Shows all the currently active breakpoints.");
                println!("* break-remove <address> - Removes an existing breakpoint at the given (hex) address, if it exists.");
                println!("* continue - Begins execution until a breakpoint is hit.");
                println!("* exit - Exits the program.");
                println!("* help - How you got here.");
                println!("* next - Displays the next instruction to be executed.");
                println!("* registers - Displays the contents of all cpu registers.");
                println!("* step - Executes a single instruction.\n");
            },
            Ok(Command::Next) => {
                // TODO: Clean this up.
                let pc = &self.gameboy.cpu.pc;
                match self.gameboy.try_read(*pc) {
                    Ok(opcode) => match Opcode::parse(opcode) {
                        Some(opcode) => {
                            match self.gameboy.try_cartridge_read_bytes(*pc, opcode.size() as u16) {
                                Ok(instruction) => {
                                    println!("{:#X}: {:#08X}", pc, instruction)
                                },
                                Err(error) => {
                                    println!("Unable to read instruction: {}", error)
                                },
                            }
                        },
                        None => println!("Unable to parse opcode: {:#04X}", opcode),
                    },
                    Err(error) => println!("Unable to read opcode: {}", error),
                }
            },
            Ok(Command::Registers) => {
                for Register { name, value } in self.gameboy.cpu.registers() {
                    println!("{name}: {value:#X}");
                }
            },
            Ok(Command::Step) => {
                match self.gameboy.tick() {
                    // Discard the cycle count because we will draw the frame after every
                    // instruction anyways.
                    Ok(_) => {},
                    Err(error) => error.realize(),
                }
            },
            Err(error) => {
                eprintln!("{error}");
            },
        }
    }

    fn should_break(&self) -> bool {
        self.breakpoints.iter().any(|bp| bp.address == self.gameboy.cpu.pc)
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
                let address = parse_address(&tokens[1])?;
                Ok(Command::BreakAdd(address))
            },
            "break-remove" if tokens.len() == 2 => {
                let address = parse_address(&tokens[1])?;
                Ok(Command::BreakRemove(address))
            },
            "break-list" if tokens.len() == 1 => Ok(Command::BreakList),
            "continue" if tokens.len() == 1 => Ok(Command::Continue),
            "exit" if tokens.len() == 1 => Ok(Command::Exit),
            "help" if tokens.len() == 1 => Ok(Command::Help),
            "next" if tokens.len() == 1 => Ok(Command::Next),
            "registers" if tokens.len() == 1 => Ok(Command::Registers),
            "step" if tokens.len() == 1 => Ok(Command::Step),
            "break-add" | "break-remove" | "break-list" | "continue" | "exit" | "help" | "next"
            | "registers" | "step" => Err(CommandParseError::InvalidFormat),
            other => Err(CommandParseError::InvalidCommand(other)),
        }
    }
}

fn parse_address(address: &str) -> Result<u16, CommandParseError<'_>> {
    u16::from_str_radix(address, 16).map_err(|_| CommandParseError::InvalidBreakpointAddress)
}
