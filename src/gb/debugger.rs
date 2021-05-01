use crate::Gameboy;
use super::opcode::Opcode;
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
        match DebuggerCommand::from(command) {
            Some(command) => {
                if !command.valid_structure() {
                    println!("{}", command.usage());
                    return;
                }
                match command.type_ {
                    DebuggerCommandType::BreakAdd => {
                        match parse_break_address(&command) {
                            Ok(address) => {
                                self.breakpoints.push(Breakpoint::new(address));
                                println!("Breakpoint added at address: {:#X}", address);
                            },
                            Err(error) => println!("{}", error),
                        }
                    },
                    DebuggerCommandType::BreakList => {
                        for breakpoint in self.breakpoints.iter() {
                            println!("{:#X}", breakpoint.address);
                        }
                    },
                    DebuggerCommandType::BreakRemove => {
                        match parse_break_address(&command) {
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
                    DebuggerCommandType::Continue => {
                        loop {
                            // TODO: Handle cycles.
                            // TODO: Handle drawing.
                            if let Err(error) = self.gameboy.tick() {
                                error.realize();
                                break;
                            }
                            if self.should_break() {
                                break;
                            }
                        }
                    },
                    DebuggerCommandType::Exit => std::process::exit(0),
                    DebuggerCommandType::Help => {
                        // TODO-CQ: Derive usage strings and descriptions from DebuggerCommandType.
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
                    DebuggerCommandType::Next => {
                        let pc = &self.gameboy.cpu.pc;
                        match self.gameboy.try_read(*pc) {
                            Ok(opcode) => {
                                match Opcode::parse(opcode) {
                                    Some(opcode) => {
                                        match self.gameboy.try_cartridge_read_bytes(*pc, opcode.size() as u16) {
                                            Ok(instruction) => println!("{:#X}: {:#08X}", pc, instruction),
                                            Err(error) => println!("Unable to read instruction: {}", error),
                                        }
                                    },
                                    None => println!("Unable to parse opcode: {:#04X}", opcode),
                                }
                            },
                            Err(error) => println!("Unable to read opcode: {}", error),
                        }
                    },
                    DebuggerCommandType::Registers => {
                        // TODO-CQ: Use an iterator?
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
                    DebuggerCommandType::Step => {
                        match self.gameboy.tick() {
                            // Discard the cycle count because we will draw the frame after every instruction anyways.
                            Ok(_) => self.gameboy.display.draw(), 
                            Err(error) => error.realize(),
                        }
                    },
                }
            },
            None => println!("Invalid command! Type \"help\" to view available commands."),
        };
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

#[derive(Clone)]
struct DebuggerCommand {
    tokens: Vec<String>,
    type_: DebuggerCommandType,
}

#[derive(Clone)]
enum DebuggerCommandType {
    BreakAdd,
    BreakList,
    BreakRemove,
    Continue,
    Exit,
    Help,
    Next,
    Registers,
    Step,
}

impl DebuggerCommand {
    fn from(command: &str) -> Option<Self> {
        let tokens: Vec<String> = command.split(' ').map(|s| String::from(s)).collect();
        let type_ = match tokens[0].as_str() {
            "break-add"    => Some(DebuggerCommandType::BreakAdd),
            "break-list"   => Some(DebuggerCommandType::BreakList),
            "break-remove" => Some(DebuggerCommandType::BreakRemove),
            "continue"     => Some(DebuggerCommandType::Continue),
            "exit"         => Some(DebuggerCommandType::Exit),
            "help"         => Some(DebuggerCommandType::Help),
            "next"         => Some(DebuggerCommandType::Next),
            "registers"    => Some(DebuggerCommandType::Registers),
            "step"         => Some(DebuggerCommandType::Step),
            _              => None,
        };
        match type_ {
            Some(type_) => Some(Self { tokens, type_ }),
            None => None,
        }
    }

    fn valid_structure(&self) -> bool {
        self.tokens.len() == match self.type_ {
            DebuggerCommandType::BreakAdd
                | DebuggerCommandType::BreakRemove => 2,
            DebuggerCommandType::BreakList
                | DebuggerCommandType::Continue
                | DebuggerCommandType::Exit
                | DebuggerCommandType::Help
                | DebuggerCommandType::Next
                | DebuggerCommandType::Registers
                | DebuggerCommandType::Step        => 1,
        }
    }

    fn usage(&self) -> String {
        let arguments = match self.type_ {
            DebuggerCommandType::BreakAdd
                | DebuggerCommandType::BreakRemove => Some("<address (ex: 0x1000)>"),
            _ => None
        };
        match arguments {
            Some(arguments) => format!("Usage: {} {}", self.type_, arguments),
            None => format!("Usage: {}", self.type_),
        }   
    }
}

impl fmt::Display for DebuggerCommandType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::BreakAdd    => "break-add",
            Self::BreakList   => "break-list",
            Self::BreakRemove => "break-remove",
            Self::Continue    => "continue",
            Self::Exit        => "exit",
            Self::Help        => "help",
            Self::Next        => "next",
            Self::Registers   => "registers",
            Self::Step        => "step",
        })
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
fn parse_break_address(command: &DebuggerCommand) -> Result<u16, ParseBreakAddressError> {
    if command.tokens.len() != 2 || command.tokens[1].len() < 3 || &command.tokens[1][..2] != "0x" {
        return Err(ParseBreakAddressError::BadUsage { command: command.clone() });
    }
    match u16::from_str_radix(&command.tokens[1][2..], 16) {
        Ok(address) => Ok(address),
        Err(error) => Err(ParseBreakAddressError::BadParse { error }),
    }
}

enum ParseBreakAddressError {
    BadUsage { command: DebuggerCommand },
    BadParse { error: std::num::ParseIntError }
}

impl fmt::Display for ParseBreakAddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::BadUsage { command } => command.usage(),
            Self::BadParse { error } => format!("Error parsing address: {}", error),
        })
    }
}
