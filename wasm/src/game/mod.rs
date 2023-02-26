pub mod fen;
pub mod notation;
pub mod pieces;
pub mod game;
pub use game::Game;

pub const MASK: u128 = 0xff00_ff00_ff00_ff00_ff00_ff00_ff00_ff00;
