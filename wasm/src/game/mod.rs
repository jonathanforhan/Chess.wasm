pub mod fen;
pub mod notation;
pub mod pieces;
pub mod game;
pub use game::Game;

pub const MASK: u128 = 0xff00_ff00_ff00_ff00_ff00_ff00_ff00_ff00;
pub const K_CASTLE: u128 = (0x10 << 0x08) | (0x80 << 0x08);
pub const Q_CASTLE: u128 = (0x10 << 0x08) | (0x01 << 0x08);
#[allow(non_upper_case_globals)]
pub const k_CASTLE: u128 = (0x10 << 0x78) | (0x80 << 0x78);
#[allow(non_upper_case_globals)]
pub const q_CASTLE: u128 = (0x10 << 0x78) | (0x01 << 0x78);
