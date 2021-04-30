use crate::{ Gameboy, TickError };
use std::fmt;

pub struct Debugger {
    pub gameboy: Gameboy,
    breakpoints: Vec<Breakpoint>,
}

impl Debugger {
    pub fn new(gameboy: Gameboy) -> Self {
        Self { gameboy, breakpoints: Vec::new() }
    }

    pub fn invoke_command(&mut self, command: &str) {
        let tokens: Vec<&str> = command.split(' ').collect();
        match tokens[0] {
            "break-add" => {
                match parse_break_address(&tokens) {
                    Ok(address) => {
                        self.breakpoints.push(Breakpoint::new(address));
                        println!("Breakpoint added at address: {:#X}", address);
                    },
                    Err(error) => println!("{}", error),
                }
            },
            "break-list" => {
                for breakpoint in self.breakpoints.iter() {
                    println!("{:#X}", breakpoint.address);
                }
            },
            "break-remove" => {
                match parse_break_address(&tokens) {
                    Ok(address) => {
                        for (idx, breakpoint) in self.breakpoints.iter().enumerate() {
                            if breakpoint.address == address {
                                self.breakpoints.remove(idx);
                                println!("Breakpoint removed at address: {:#X}", address);
                                return;
                            }
                        }
                        println!("No breakpoint exists at address: {:#X}", address);
                    },
                    Err(error) => println!("{}", error),
                }
            },
            "continue" => {
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
            "exit" => std::process::exit(0),
            "help" => {
                println!("---------------");
                println!(" Boyo Debugger");
                println!("---------------\n");
                println!("Commands");
                println!("* break-add <address> - Adds a new breakpoint at the given (hex) address.");
                println!("* break-list - Shows all the currently active breakpoints.");
                println!("* break-remove <address> - Removes an existing breakpoint at the given (hex) address, if it exists.");
                println!("* continue - Begins execution until a breakpoint is hit.");
                println!("* exit - Exits the program.");
                println!("* help - How you got here.");
                println!("* next - Displays the next instruction to be executed.");
                println!("* registers - Displays the contents of all cpu registers.");
                println!("* step - Executes a single instruction.\n");
            },
            "next" => {
                let pc = &self.gameboy.cpu.pc;
                match self.gameboy.try_read(*pc) {
                    Ok(value) => println!("{:#X}: {:#04X}", pc, value),
                    Err(error) => println!("{}", error),
                }
            },
            "registers" => {
                let cpu = &self.gameboy.cpu;
                println!("a: {:#X}", cpu.a);
                println!("b: {:#X}", cpu.b);
                println!("c: {:#X}", cpu.c);
                println!("d: {:#X}", cpu.d);
                println!("e: {:#X}", cpu.e);
                println!("f: {:#X}", cpu.f);
                println!("h: {:#X}", cpu.h);
                println!("l: {:#X}", cpu.l);
            }
            "step" => {
                match self.step() {
                    Ok(_) => (),
                    Err(error) => error.realize(),
                }
            },
            _ => println!("Invalid command! Type \"help\" to view available commands."),
        }
    }

    fn step(&mut self) -> Result<(), TickError> {
        // Discard the cycle count because we will draw the frame after every instruction anyways.
        self.gameboy.tick()?;
        Ok(())
    }

    fn should_break(&self) -> bool {
        for breakpoint in self.breakpoints.iter() {
            if self.gameboy.cpu.pc == breakpoint.address {
                return true;
            }
        }
        false
    }
}

struct Breakpoint {
    address: u16,
}

impl Breakpoint {
    pub fn new(address: u16) -> Self {
        Self { address }
    }
}

// This function is simply for parsing the break-add and break-remove commands, as they share a common structure (break-[add|remove] <address (hex)>).
fn parse_break_address(tokens: &Vec<&str>) -> Result<u16, ParseBreakAddressError> {
    if tokens.len() != 2 || tokens[1].len() < 3 || &tokens[1][..2] != "0x" {
        return Err(ParseBreakAddressError::BadUsage { command: String::from(tokens[0]) });
    }
    match u16::from_str_radix(&tokens[1][2..], 16) {
        Ok(address) => Ok(address),
        Err(error) => Err(ParseBreakAddressError::BadParse { error }),
    }
}

enum ParseBreakAddressError {
    BadUsage { command: String },
    BadParse { error: std::num::ParseIntError }
}

impl fmt::Display for ParseBreakAddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::BadUsage { command } => format!("Usage: {} <address (ex: 0x1000)>", command),
            Self::BadParse { error } => format!("Error parsing address: {}", error),
        })
    }
}
