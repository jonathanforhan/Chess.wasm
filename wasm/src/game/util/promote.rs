use crate::game::pieces::{
    Color,
    Color::White,
    Color::Black,
    Piece,
    Pieces,
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
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

#[inline]
pub fn add_promotions(mv: &Pieces, moves: &mut Vec<Pieces>) {
    let bits = mv.bits();
    match mv.color() {
        White => {
            if bits & WHITE_BACK_RANK == 0 { return; }
            moves.push(Pieces::Pawn(Pawn::from_bits(bits | WHITE_ROOK, White)));
            moves.push(Pieces::Pawn(Pawn::from_bits(bits | WHITE_KNIGHT, White)));
            moves.push(Pieces::Pawn(Pawn::from_bits(bits | WHITE_BISHOP, White)));
        },
        Black => {
            if bits & BLACK_BACK_RANK == 0 { return; }
            moves.push(Pieces::Pawn(Pawn::from_bits(bits | BLACK_ROOK, Black)));
            moves.push(Pieces::Pawn(Pawn::from_bits(bits | BLACK_KNIGHT, Black)));
            moves.push(Pieces::Pawn(Pawn::from_bits(bits | BLACK_BISHOP, Black)));
        }
    }
}

#[inline]
pub fn try_promote (piece: &mut Pieces, mv: &mut u128, turn: Color) {
    match turn {
        White => {
            if *mv & WHITE_BACK_RANK == 0 { return; }
            let id = *mv & BLACK_BACK_RANK;
            match id {
                WHITE_BISHOP => {
                    *piece = Pieces::Bishop(Bishop::from_bits(*piece.bits(), White));
                    *mv &= !WHITE_BISHOP;
                },
                WHITE_KNIGHT => {
                    *piece = Pieces::Knight(Knight::from_bits(*piece.bits(), White));
                    *mv &= !WHITE_KNIGHT;
                },
                WHITE_ROOK => {
                    *piece = Pieces::Rook(Rook::from_bits(*piece.bits(), White));
                    *mv &= !WHITE_ROOK;
                },
                _ => *piece = Pieces::Queen(Queen::from_bits(*piece.bits(), White))
            }
        },
        Black => {
            if *mv & BLACK_BACK_RANK == 0 { return; }
            let id = *mv & WHITE_BACK_RANK;
            match id {
                BLACK_BISHOP => {
                    *piece = Pieces::Bishop(Bishop::from_bits(*piece.bits(), Black));
                    *mv &= !BLACK_BISHOP;
                },
                BLACK_KNIGHT => {
                    *piece = Pieces::Knight(Knight::from_bits(*piece.bits(), Black));
                    *mv &= !BLACK_KNIGHT;
                },
                BLACK_ROOK => {
                    *piece = Pieces::Rook(Rook::from_bits(*piece.bits(), Black));
                    *mv &= !BLACK_ROOK;
                },
                _ => *piece = Pieces::Queen(Queen::from_bits(*piece.bits(), Black))
            }
        }
    }
}
