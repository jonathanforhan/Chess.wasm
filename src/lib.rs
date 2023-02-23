mod pieces;
pub use pieces::{
    BitBoard,
    Piece,
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
    Color,
};
mod chess;
mod fen;

#[cfg(test)]
mod test;
