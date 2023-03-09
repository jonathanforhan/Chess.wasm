use crate::game::pieces::{
    Color,
    Color::White,
    Color::Black,
    Piece,
    Pieces,
    Pawn,
};

pub mod constants {
    // Promotion indicators are tacked onto
    // pawn promtion moves and used to determine
    // the desired promtion piece.
    // No indicator is a Queen promotion

    // White promotion option indicators
    pub const WHITE_ROOK: u128 = 0x0100;   // x.......
    pub const WHITE_KNIGHT: u128 = 0x0200; // .x......
    pub const WHITE_BISHOP: u128 = 0x0400; // ..x.....
    pub const WHITE_BACK_RANK: u128 = 0xff00 << 0x70;

    // Black promotion option indicators
    pub const BLACK_ROOK: u128 = 0x0100 << 0x70;   // x.......
    pub const BLACK_KNIGHT: u128 = 0x0200 << 0x70; // .x......
    pub const BLACK_BISHOP: u128 = 0x0400 << 0x70; // ..x.....
    pub const BLACK_BACK_RANK: u128 = 0xff00;
}
pub use constants::*;

pub fn add_promotions(mvs: &Vec<Pieces>, color: Color) -> Vec<Pieces> {
    let mut moves = Vec::new();
    if color == White {
        for mv in mvs {
            if mv.bits() & WHITE_BACK_RANK == 0 { continue; }
            moves.push(Pieces::Pawn(Pawn::from_bits(mv.bits() | WHITE_ROOK, White)));
            moves.push(Pieces::Pawn(Pawn::from_bits(mv.bits() | WHITE_KNIGHT, White)));
            moves.push(Pieces::Pawn(Pawn::from_bits(mv.bits() | WHITE_BISHOP, White)));
        }
    } else { // Black
        for mv in mvs {
            if mv.bits() & BLACK_BACK_RANK == 0 { continue; }
            moves.push(Pieces::Pawn(Pawn::from_bits(mv.bits() | BLACK_ROOK, Black)));
            moves.push(Pieces::Pawn(Pawn::from_bits(mv.bits() | BLACK_KNIGHT, Black)));
            moves.push(Pieces::Pawn(Pawn::from_bits(mv.bits() | BLACK_BISHOP, Black)));
        }
    }
    return moves;
}
