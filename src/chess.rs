use wasm_bindgen::prelude::*;
use super::*;

#[wasm_bindgen]
pub struct Chess {
    pieces: Vec::<BitBoard>,
    // game: Game,
}

#[wasm_bindgen]
impl Chess {
    #[wasm_bindgen(constructor)]
    pub fn new(fen: String) -> Self {
        let mut chess = Chess { pieces: Vec::new() };
        chess.load(fen);
        return chess;
    }

    pub fn load(&mut self, fen: String) {

    }

    pub fn fen(self) -> String {
        "".to_string()
    }

    pub fn clear() {
        
    }
}

