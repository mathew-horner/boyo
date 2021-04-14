use clap::{App, Arg};
use std::fs::File;

fn main() {
    let matches = App::new("boyo")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Mathew Horner <mathewhorner456@gmail.com>")
        .about("A cycle-accurate, efficient, and memory safe emulator for the Gameboy and Gameboy Advance.")
        .arg(Arg::with_name("rom")
            .required(true)
            .index(1))
        .get_matches();
    
    let rom = matches.value_of("rom").unwrap();
    let file = match File::open(rom) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(0);
        }
    };
}
