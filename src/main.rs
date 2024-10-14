mod command_history;
mod debugger;
mod gb;
mod instruction;

use std::num::ParseIntError;

use clap::{ArgGroup, Parser};

use crate::debugger::{run_terminal_debugger, Debugger};
use crate::gb::Gameboy;

#[derive(Parser)]
#[command(
    version,
    author = "Mathew Horner <mathewhorner456@gmail.com>",
    about = "A cycle-accurate, efficient, and memory safe emulator for the Gameboy and Gameboy Advance."
)]
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .args(&["rom_path", "raw"])
))]
struct Cli {
    /// Path to a Gameboy ROM file.
    rom_path: Option<String>,

    /// Launch in command line debug mode.
    #[arg(short, long, default_value_t = false)]
    debug: bool,

    #[arg(short, long, value_parser = parse_u8)]
    /// Raw ROM data to run with.
    raw: Option<Vec<u8>>,
}

fn parse_u8(input: &str) -> Result<u8, ParseIntError> {
    if let Some(hex_str) = input.strip_prefix("0x") {
        u8::from_str_radix(hex_str, 16)
    } else {
        input.parse::<u8>()
    }
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    let rom = match cli.rom_path {
        Some(path) => std::fs::read(&path).expect("failed to read ROM file"),
        None => {
            // The system does not start execution at address=0, so we need to pad 0s until
            // we reach the starting position of the PC.
            let mut rom = vec![0; crate::gb::INITIAL_PC as usize];

            // raw should never be None here because we put it in an ArgGroup with
            // rom_path. If rom_path is not provided, raw should be required.
            rom.extend(cli.raw.unwrap());

            rom
        },
    };
    let gameboy = Gameboy::new(rom);

    if !cli.debug {
        log::info!("Starting boyo in execution mode");
        gameboy.execute();
    } else {
        log::info!("Starting boyo in terminal debug mode");
        let debugger = Debugger::new(gameboy);
        run_terminal_debugger(debugger);
    }
}
