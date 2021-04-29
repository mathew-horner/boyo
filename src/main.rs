mod gb;
use clap::{App, Arg};
use gb::{Gameboy, Cartridge, Debugger, TickError};
use std::io::Write;

fn main() {
    let matches = App::new("boyo")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Mathew Horner <mathewhorner456@gmail.com>")
        .about("A cycle-accurate, efficient, and memory safe emulator for the Gameboy and Gameboy Advance.")
        .arg(Arg::with_name("rom")
            .required(true)
            .index(1))
        .arg(Arg::with_name("debug")
            .long("debug")
            .short("d")
            .required(false)
            .takes_value(false))
        .get_matches();

    let path = matches.value_of("rom").unwrap();
    let mut gameboy = Gameboy::new(Cartridge::from(path).unwrap());

    if !matches.is_present("debug") {
        #[allow(while_true)]
        while true {
            match gameboy.tick() {
                Ok(_cycles) => (),
                Err(error) => {
                    println!("{}", error);
                    if !error.recoverable() {
                        std::process::exit(0);
                    }
                }
            }
        }
    } else {
        let mut debugger = Debugger::new(gameboy);
        #[allow(while_true)]
        while true {
            print!("> ");
            let _ = std::io::stdout().flush();
            let mut command = String::new();
            std::io::stdin().read_line(&mut command).expect("Failed to read command for debugger!");
            match command.trim() {
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
                    let pc = &debugger.gameboy.cpu.pc;
                    match debugger.gameboy.try_read(*pc) {
                        Ok(value) => println!("{:#X}: {:#04X}", pc, value),
                        Err(error) => println!("{}", error),
                    }
                },
                "registers" => {
                    let cpu = &debugger.gameboy.cpu;
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
                    match debugger.step() {
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
                _ => println!("Error: Invalid command!"),
            }
        }
    }
}