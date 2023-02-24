use wasm_bindgen::prelude::*;
use js_sys;

use super::*;

/* Chess struct is essentially a JS wrapper on game */
#[wasm_bindgen]
pub struct Chess {
    fen: String,
    game: Game,
}

#[wasm_bindgen]
impl Chess {
    #[wasm_bindgen(constructor)]
    pub fn new(fen: &str) -> Self {
        let game = fen::decode(fen).unwrap_or(
            fen::decode("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1").unwrap()
        );
        Chess { fen: fen.to_string(), game }
    }

    pub fn load(&mut self, fen: &str) {
        let game = fen::decode(fen).unwrap_or(
            fen::decode("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1").unwrap()
        );
        self.game = game;
    }

    pub fn board(&mut self) -> js_sys::Array {
        let array: Vec<f64> = self.game.pieces.iter_mut().map(|x| {
            BitBoard::convert_to_64bit(x.bits().unwrap_throw()) as f64
        }).collect();

        let js_array: js_sys::Array = js_sys::Array::new();
        for n in array {
            js_array.push(&JsValue::from_f64(n));
        };

        js_array
    }

    pub fn fen(&self) -> String {
        self.fen.clone()
    }

    pub fn moves(&self) -> js_sys::Array {
        let mvs = &self.game.moves();
        let an: Vec<String> = mvs.into_iter().map(|x| bits_to_algebraic(x.bits().unwrap_throw()).unwrap_throw()).collect();
        let js_array: js_sys::Array = js_sys::Array::new();
        for a in an {
            js_array.push(&JsValue::from_str(&a[..]));
        }
        js_array
    }
}

