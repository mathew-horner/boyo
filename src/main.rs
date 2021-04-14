mod common;
mod gb;
mod gba;
use clap::{App, Arg};
use common::Emulator;
use gb::{Gameboy, GameboyCartridge};
use gba::{GameboyAdvance, GameboyAdvanceCartridge};

const SYSTEM_GB: &str = "GB";
const SYSTEM_GBA: &str = "GBA";

fn main() {
    let matches = App::new("boyo")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Mathew Horner <mathewhorner456@gmail.com>")
        .about("A cycle-accurate, efficient, and memory safe emulator for the Gameboy and Gameboy Advance.")
        .arg(Arg::with_name("rom")
            .required(true)
            .index(1))
        .arg(Arg::with_name("system")
            .required(true))
        .get_matches();

    // TODO: infer system type based on ROM data?
    let path = matches.value_of("rom").unwrap();
    let system = matches.value_of("system").unwrap().to_ascii_uppercase();
    // TODO: use enum variants for exhaustive matching, find a way to generalize this code better.
    match system.as_str() {
        SYSTEM_GB => { 
            Gameboy::new(GameboyCartridge::from(path).unwrap()).start()
        },
        SYSTEM_GBA => {
            GameboyAdvance::new(GameboyAdvanceCartridge::from(path).unwrap()).start()
        },
        _ => {
            println!("Error: {} is not a valid system!", system);
            std::process::exit(0);
        }
    }
}