mod gb;
use clap::{App, Arg};
use gb::{Gameboy, Cartridge, Debugger, TickError};
use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::thread::sleep;

// This value is used to throttle the amount of instructions that are executed every frame.
const CYCLES_PER_FRAME: u32 = 70_224;

// This value is used to lock the frame rate at a given frequency.
const REFRESH_RATE: f64 = 59.73;

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
        // Ideally, this would be a constant but Rust doesn't support this as a constant function yet.
        let frame_duration = Duration::from_secs_f64(1.0 / REFRESH_RATE);

        #[allow(while_true)]
        while true {
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
            sleep(frame_duration.checked_sub(start.elapsed()).unwrap());
        }
    } else {
        let mut debugger = Debugger::new(gameboy);
        #[allow(while_true)]
        while true {
            print!("> ");
            let _ = io::stdout().flush();
            let mut command = String::new();
            io::stdin()
                .read_line(&mut command)
                .expect("Failed to read command for debugger!");
                
            debugger.invoke_command(command.trim());
        }
    }
}
