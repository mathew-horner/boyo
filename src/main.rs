mod gameboy;
use clap::{App, Arg};
use gameboy::{Gameboy, GameboyCartridge};

fn main() {
    let matches = App::new("boyo")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Mathew Horner <mathewhorner456@gmail.com>")
        .about("A cycle-accurate, efficient, and memory safe emulator for the Gameboy and Gameboy Advance.")
        .arg(Arg::with_name("rom")
            .required(true)
            .index(1))
        .get_matches();
    // TODO: infer system type based on ROM data?
    let path = matches.value_of("rom").unwrap();
    let cartridge = match GameboyCartridge::from(path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(0);
        }
    };
    let gameboy = Gameboy::new(cartridge);
    #[allow(while_true)]
    while true {
        gameboy.tick();
    }
}
