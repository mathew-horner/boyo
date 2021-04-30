use crate::{ Gameboy, TickError };

pub struct Debugger {
    pub gameboy: Gameboy,
}

impl Debugger {
    pub fn new(gameboy: Gameboy) -> Self {
        Self { gameboy }
    }

    pub fn invoke_command(&mut self, command: &str) {
        match command {
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
}