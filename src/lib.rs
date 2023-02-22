use wasm_bindgen::prelude::*;

mod pieces;
use pieces::BitBoards;
use pieces::{
    Piece,
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
};

#[wasm_bindgen]
pub struct Chess {
    fen: String,
    pieces: Vec::<BitBoards>,
}

#[wasm_bindgen]
impl Chess {
    #[wasm_bindgen(constructor)]
    pub fn new(fen: String) -> Self {
        Chess { fen , pieces: Vec::new() }
    }

    pub fn fen(self) -> String {
        self.fen
    }

    pub fn add_piece(&mut self) {
        self.pieces.push(BitBoards::Pawn(Pawn::new(0, 0)))
    }
}
