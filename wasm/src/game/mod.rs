use std::{
    error::Error,
    fmt
};

pub mod fen;
pub mod notation;
pub mod pieces;
pub mod game;
pub use game::Game;
pub mod util;

// used for off-board detection
pub const MASK: u128 = 0xff00_ff00_ff00_ff00_ff00_ff00_ff00_ff00;

#[derive(Debug)]
pub struct GameError(String);

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Game Error: {}", self.0)
    }
}

impl Error for GameError {}
