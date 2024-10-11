mod gb_pca;

use clap::Parser;
use gb_pca::run_terminal_debugger;

use crate::gb_pca::{Debugger, Gameboy};

#[derive(Parser)]
#[command(
    version,
    author = "Mathew Horner <mathewhorner456@gmail.com>",
    about = "A cycle-accurate, efficient, and memory safe emulator for the Gameboy and Gameboy Advance."
)]
struct Cli {
    /// Path to a Gameboy ROM file.
    rom_path: String,

    /// Launch in command line debug mode.
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

fn main() {
    let cli = Cli::parse();
    let rom = std::fs::read(&cli.rom_path).expect("failed to read ROM file");
    let gameboy = Gameboy::new(rom);

    if !cli.debug {
        gameboy.execute();
    } else {
        let debugger = Debugger::new(gameboy);
        run_terminal_debugger(debugger);
    }
}
