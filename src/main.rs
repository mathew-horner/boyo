mod gb;
mod gb_pca;

use std::io::{self, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

use clap::Parser;
use console::Term;

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
        let term = Term::stdout();
        loop {
            print!("> ");
            let _ = io::stdout().flush();
            let mut command = String::new();

            // history_idx = 0 refers to the command buffer. history_idx = 1.. refers to
            // history @ history_idx - 1.
            let mut history_idx: usize = 0;

            loop {
                match term.read_key().unwrap() {
                    console::Key::Enter => {
                        println!();
                        break;
                    },
                    console::Key::Backspace => {
                        let _ = command.pop();
                        term.clear_chars(1).unwrap();
                    },
                    console::Key::ArrowUp if history_idx < debugger.history_len() => {
                        history_idx += 1;
                        update_command_display(&debugger, &term, history_idx, &command);
                    },
                    console::Key::ArrowDown if history_idx > 0 => {
                        history_idx -= 1;
                        update_command_display(&debugger, &term, history_idx, &command);
                    },
                    console::Key::Char(char) => {
                        print!("{char}");
                        io::stdout().flush().unwrap();
                        command.push(char);
                    },
                    _ => {},
                };
            }

            // TODO: Make it so we don't have to clone here...
            let command = match history_idx {
                0 => command.trim().to_owned(),
                idx => debugger.history_entry(idx - 1).unwrap().to_owned(),
            };

            debugger.invoke_command(&command);
        }
    }
}

fn update_command_display(debugger: &Debugger, term: &Term, history_idx: usize, command: &str) {
    term.clear_line().unwrap();

    let value = match history_idx {
        0 => command,
        // We never let history_idx outside the bounds of the history, so this
        // unwrap is safe.
        idx => debugger.history_entry(idx - 1).unwrap(),
    };

    print!("> {value}");
    io::stdout().flush().unwrap();
}
