pub mod fen;
pub mod notation;
pub mod pieces;
pub mod game;
pub use game::Game;

// used for off-board detection
pub const MASK: u128 = 0xff00_ff00_ff00_ff00_ff00_ff00_ff00_ff00;

#[allow(non_upper_case_globals)]
pub mod castle {

    // These are the location of the king and rook squares
    // used to detect if they moved, so we can take away their
    // castling rights
    pub const K_SQUARES: u128 = 0x90 << 0x08; // ....x..x
    pub const Q_SQUARES: u128 = 0x11 << 0x08; // x...x...
    pub const k_SQUARES: u128 = 0x90 << 0x78; // ....x..x
    pub const q_SQUARES: u128 = 0x11 << 0x78; // x...x...

    // This is the move the king maskes when castling
    // on each side of the baord
    // used in both user input and when the king moves
    pub const K_MOVE: u128 = 0x50 << 0x08; // ....x.x.
    pub const Q_MOVE: u128 = 0x14 << 0x08; // ..x.x...
    pub const k_MOVE: u128 = 0x50 << 0x78; // ....x.x.
    pub const q_MOVE: u128 = 0x14 << 0x78; // ..x.x...

    // used to distinguish a castle in move list from
    // normal move, hits both king and rook
    pub const K_ZONE: u128 = 0xf0 << 0x08; // ....xxxx
    pub const Q_ZONE: u128 = 0x1f << 0x08; // xxxxx...
    pub const k_ZONE: u128 = 0xf0 << 0x78; // ....xxxx
    pub const q_ZONE: u128 = 0x1f << 0x78; // xxxxx...
}
