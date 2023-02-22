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

pub enum BitBoards {
    Pawn(Pawn),
    Bishop(Bishop),
    Knight(Knight),
    Rook(Rook),
    Queen(Queen),
    King(King),
    //White(White),
    //Black(Black),
}

// implimented by unique structs //
pub trait Piece {
    type T: Piece;
    fn new(x: usize, y: usize) -> Self::T;
    fn moves(&self, opp: &u128, team: &u128) -> Vec<Self::T>;
}
