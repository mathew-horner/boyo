use crate::{ Gameboy, TickError };

pub struct Debugger {
    pub gameboy: Gameboy,
}

impl Debugger {
    pub fn new(gameboy: Gameboy) -> Self {
        Self { gameboy }
    }

    pub fn step(&mut self) -> Result<(), TickError> {
        // Discard cycle count because we don't care about it when in debug mode.
        match self.gameboy.tick() {
            Ok(_) => Ok(()),
            Err(error) => Err(error)
        }
    }
}