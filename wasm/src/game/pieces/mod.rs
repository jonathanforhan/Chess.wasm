use enum_dispatch::enum_dispatch;

pub mod pawn;
pub use pawn::Pawn;
pub mod bishop;
pub use bishop::Bishop;
pub mod knight;
pub use knight::Knight;
pub mod rook;
pub use rook::Rook;
pub mod queen;
pub use queen::Queen;
pub mod king;
pub use king::King;

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[enum_dispatch(Piece)]
pub enum Pieces {
    Pawn(Pawn),
    Bishop(Bishop),
    Knight(Knight),
    Rook(Rook),
    Queen(Queen),
    King(King),
}

#[enum_dispatch]
pub trait Piece {
    fn bits(&self) -> &u128;
    fn set_bits(&mut self, bits: &u128);
    fn color(&self) -> &Color;
    fn moves(&self, opp: &u128, team: &u128) -> Vec<Pieces>;
}
