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
                match Self::parse_break_address(&tokens) {
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
                match Self::parse_break_address(&tokens) {
                    Ok(address) => {
                        // TODO-CQ: Could probably use a more FP approach here.
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
            "exit" => std::process::exit(0),
            "help" => {
                println!("---------------");
                println!(" Boyo Debugger");
                println!("---------------\n");
                println!("Commands");
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
                    Err(error) => {
                        // TODO: Find a way to make this a function?
                        println!("{}", error);
                        if !error.recoverable() {
                            std::process::exit(0);
                        }
                    }
                }
            },
            _ => println!("Invalid command! Type \"help\" to view available commands."),
        }
    }

    fn step(&mut self) -> Result<(), TickError> {
        // Discard cycle count because we don't care about it when in debug mode.
        self.gameboy.tick()?;
        Ok(())
    }

    fn parse_break_address(tokens: &Vec<&str>) -> Result<u16, ParseBreakAddressError> {
        if tokens.len() != 2 || tokens[1].len() < 3 || &tokens[1][..2] != "0x" {
            return Err(ParseBreakAddressError::BadUsage { command: String::from(tokens[0]) });
        }
        match u16::from_str_radix(&tokens[1][2..], 16) {
            Ok(address) => Ok(address),
            Err(error) => Err(ParseBreakAddressError::BadParse { error }),
        }
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