use crate::game::pieces::{
    Color,
    Color::White,
    Color::Black,
    Piece,
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

    // Castling is still possible when b8 and b1
    // are under attack by opposing pieces, hence
    // edge case
    pub const EDGE_CASE: u128 = (0x02 << 0x08) | (0x02 << 0x78);
}
pub use constants::*;

pub fn add_castling(castling: &String, obstacles: &u128, color: Color, moves: &mut Vec<Pieces>) {
    if color == White {
        if castling.contains('K') && K_VALID & obstacles == 0 {
            moves.push(Pieces::King(King::from_bits(K_ZONE, White)));
        }
        if castling.contains('Q') && Q_VALID & obstacles == 0 {
            moves.push(Pieces::King(King::from_bits(Q_ZONE, White)));
        }
    } else { // Black
        if castling.contains('k') && k_VALID & obstacles == 0 {
            moves.push(Pieces::King(King::from_bits(k_ZONE, Black)));
        }
        if castling.contains('q') && q_VALID & obstacles == 0 {
            moves.push(Pieces::King(King::from_bits(q_ZONE, Black)));
        }
    }
}

pub fn try_castle (piece: &mut Pieces, king_move: u128, rook_move: u128) {
    if let Pieces::King(k) = piece {
        k.set_bits(&(k.bits() ^ king_move));
    } else if let Pieces::Rook(r) = piece {
        r.set_bits(&(r.bits() ^ rook_move));
    }
}

pub fn fix_castle (castling: &mut String, mv: &u128) {
    let mut remove_castle = |c: char, castle: u128| {
        if castling.contains(c) && mv & castle != 0 {
            castling.remove(castling.find(c).unwrap());
            if castling.len() == 0 { *castling = String::from("-"); }
        }
    };
    remove_castle('K', K_SQUARES);
    remove_castle('Q', Q_SQUARES);
    remove_castle('k', k_SQUARES);
    remove_castle('q', q_SQUARES);
}
