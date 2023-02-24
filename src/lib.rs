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
    EnPassant,
    Color,
};
mod chess;
use chess::Chess;
mod fen;
mod game;
pub use game::Game;
mod notation;
use notation::algebraic::bits_to_algebraic;

#[cfg(test)]
mod test;

const MASK: u128 = 0xff00_ff00_ff00_ff00_ff00_ff00_ff00_ff00;
