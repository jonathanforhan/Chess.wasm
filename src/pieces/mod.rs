mod pawn;
mod bishop;
mod knight;
mod rook;
mod queen;
mod king;

pub use {
    pawn::Pawn,
    bishop::Bishop,
    knight::Knight,
    rook::Rook,
    queen::Queen,
    king::King,
};

pub enum BitBoard {
    Pawn(Pawn),
    Bishop(Bishop),
    Knight(Knight),
    Rook(Rook),
    Queen(Queen),
    King(King),
}

impl BitBoard {
    fn convert_to_64bit(bits: &u128) -> u64 {
        let mut result: u64 = 0;
        for i in 0..8 {
            result |= (((bits >> (i << 4) + 8) & 0x00ff) << (i << 3)) as u64;
        }
        return result;
    }
}

#[derive(Clone, Copy)]
pub enum Color {
    White,
    Black,
}

// implimented by enum params //
pub trait Piece {
    type T: Piece;
    fn new(x: usize, y: usize, color: Color) -> Self::T;
    fn bits(&self) -> &u128;
    fn moves(&self, opp: &u128, team: &u128) -> Vec<Self::T>;
}
