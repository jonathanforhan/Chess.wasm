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

pub fn add_promotions(color: Color, moves: &mut Vec<Pieces>) {
    let mut promoting: Option<[Pieces; 3]> = None;
    if color == White {
        for mv in moves.iter() {
            if mv.bits() & WHITE_BACK_RANK == 0 { continue; }
            promoting = Some([
                Pieces::Pawn(Pawn::from_bits(mv.bits() | WHITE_ROOK, White)),
                Pieces::Pawn(Pawn::from_bits(mv.bits() | WHITE_KNIGHT, White)),
                Pieces::Pawn(Pawn::from_bits(mv.bits() | WHITE_BISHOP, White))
            ]);
        }
    } else { // Black
        for mv in moves.iter() {
            if mv.bits() & BLACK_BACK_RANK == 0 { continue; }
            promoting = Some([
                Pieces::Pawn(Pawn::from_bits(mv.bits() | BLACK_ROOK, Black)),
                Pieces::Pawn(Pawn::from_bits(mv.bits() | BLACK_KNIGHT, Black)),
                Pieces::Pawn(Pawn::from_bits(mv.bits() | BLACK_BISHOP, Black))
            ]);
        }
    }
    if let Some(promoting) = promoting {
        moves.extend_from_slice(&promoting[..]);
    }
}

pub fn try_promote (piece: &mut Pieces, mv: &u128, turn: Color) {
    if mv & (BLACK_BACK_RANK | WHITE_BACK_RANK) == 0 { return; }
    if turn == White {
        let mv = mv & BLACK_BACK_RANK; // isolate identifier bit
        match mv {
            WHITE_BISHOP => *piece = Pieces::Bishop(Bishop::from_bits(*piece.bits(), White)),
            WHITE_KNIGHT => *piece = Pieces::Knight(Knight::from_bits(*piece.bits(), White)),
            WHITE_ROOK => *piece = Pieces::Rook(Rook::from_bits(*piece.bits(), White)),
            _ => *piece = Pieces::Queen(Queen::from_bits(*piece.bits(), White))
        }
    } else { // Black
        let mv = mv & WHITE_BACK_RANK; // isolate identifier bit
        match mv {
            BLACK_BISHOP => *piece = Pieces::Bishop(Bishop::from_bits(*piece.bits(), Black)),
            BLACK_KNIGHT => *piece = Pieces::Knight(Knight::from_bits(*piece.bits(), Black)),
            BLACK_ROOK => *piece = Pieces::Rook(Rook::from_bits(*piece.bits(), Black)),
            _ => *piece = Pieces::Queen(Queen::from_bits(*piece.bits(), Black))
        }
    }
}
