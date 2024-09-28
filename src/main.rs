mod gb;

use std::io::{self, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

use clap::Parser;

use crate::gb::{Cartridge, Debugger, Gameboy};

// TODO: Not sure 3 year ago me actually knew what "cycle accurate" means, but
// today me might be even dumber...
const CYCLES_PER_FRAME: u32 = 70_224;

const REFRESH_RATE: f64 = 59.73;

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
    let mut gameboy = Gameboy::new(Cartridge::from(&cli.rom_path).unwrap());

    if !cli.debug {
        let frame_duration = Duration::from_secs_f64(1.0 / REFRESH_RATE);

        loop {
            let start = Instant::now();
            let mut frame_cycles = 0;
            while frame_cycles < CYCLES_PER_FRAME {
                match gameboy.tick() {
                    Ok(cycles) => frame_cycles += cycles as u32,
                    Err(error) => error.realize(),
                }
            }
            // TODO: Actually draw frame.
            println!("Draw frame");
            sleep(frame_duration.checked_sub(start.elapsed()).unwrap_or(Duration::ZERO));
        }
    } else {
        let mut debugger = Debugger::new(gameboy);
        loop {
            print!("> ");
            let _ = io::stdout().flush();
            let mut command = String::new();
            io::stdin().read_line(&mut command).expect("Failed to read command for debugger!");
            debugger.invoke_command(command.trim());
        }
    }
}
