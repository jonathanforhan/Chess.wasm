use crate::game::pieces::{
    Color,
    Color::White,
    Color::Black,
    Pieces,
    King,
};

#[allow(non_upper_case_globals)]
pub mod constants
{
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

    // corresponding rook castling move
    pub const K_ROOK: u128 = 0xa0 << 0x08; // .....x.x
    pub const Q_ROOK: u128 = 0x09 << 0x08; // x..x....
    pub const k_ROOK: u128 = 0xa0 << 0x78; // .....x.x
    pub const q_ROOK: u128 = 0x09 << 0x78; // x..x....

    // used to distinguish a castle in move list from
    // normal move, hits both king and rook
    pub const K_ZONE: u128 = 0xf0 << 0x08; // ....xxxx
    pub const Q_ZONE: u128 = 0x1f << 0x08; // xxxxx...
    pub const k_ZONE: u128 = 0xf0 << 0x78; // ....xxxx
    pub const q_ZONE: u128 = 0x1f << 0x78; // xxxxx...

    // used to determine if space between caslting
    // is empty and not under attack
    pub const K_VALID: u128 = 0x60 << 0x08; // .....xx.
    pub const Q_VALID: u128 = 0x0e << 0x08; // .xxx....
    pub const k_VALID: u128 = 0x60 << 0x78; // .....xx.
    pub const q_VALID: u128 = 0x0e << 0x78; // .xxx....
}
pub use constants::*;

pub fn add_castling(castling: &String, obstacles: &u128, color: Color) -> Vec<Pieces>{
    let mut result = Vec::new();
    if color == White {
        if castling.contains('K') && K_VALID & obstacles == 0 {
            result.push(Pieces::King(King::from_bits(K_ZONE, White)));
        }
        if castling.contains('Q') && Q_VALID & obstacles == 0 {
            result.push(Pieces::King(King::from_bits(Q_ZONE, White)));
        }
    } else { // Black
        if castling.contains('k') && k_VALID & obstacles == 0 {
            result.push(Pieces::King(King::from_bits(k_ZONE, Black)));
        }
        if castling.contains('q') && q_VALID & obstacles == 0 {
            result.push(Pieces::King(King::from_bits(q_ZONE, Black)));
        }
    }
    return result;
}
