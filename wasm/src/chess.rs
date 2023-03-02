use wasm_bindgen::prelude::*;
use js_sys;

use super::game::{
    Game,
    pieces::Piece,
    fen,
    notation::{
        algebraic_to_bits,
        bits_to_algebraic
    }
};

#[wasm_bindgen]
pub fn validate(fen: &str) -> Result<(), JsError> {
    match fen::validate(fen) {
        Ok(_) => Ok(()),
        Err(e) => return Err(JsError::new(&format!("{}", e)))
    }
}

#[wasm_bindgen]
pub fn moves(fen: &str) -> Result<js_sys::Array, JsError> {
    let game: Game = match fen::decode(fen) {
        Ok(g) => g,
        Err(e) => return Err(JsError::new(&format!("{}", e)))
    };
    let mut current: u128 = 0;
    for p in &game.pieces {
        if p.color() == &game.turn {
            current |= p.bits();
        }
    }

    let arr = js_sys::Array::new();

    let mvs = game.moves();
    for m in mvs {
        let src = current & m.bits(); // find the matching starting location
        let dst = m.bits() ^ src;     // subtract starting pos from move map

        let (from, to) = (bits_to_algebraic(&src)?, bits_to_algebraic(&dst)?);
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"from".into(), &JsValue::from_str(&from)).unwrap_throw();
        js_sys::Reflect::set(&obj, &"to".into(), &JsValue::from_str(&to)).unwrap_throw();
        arr.push(&obj);
    }

    Ok(arr)
}

#[wasm_bindgen]
pub fn move_piece(fen: &str, obj: js_sys::Object) -> Result<String, JsError> {
    let mut game: Game = match fen::decode(fen) {
        Ok(g) => g,
        Err(e) => return Err(JsError::new(&format!("{}", e)))
    };
    assert!(js_sys::Reflect::has(&obj, &"from".into()).unwrap());
    assert!(js_sys::Reflect::has(&obj, &"to".into()).unwrap());
    let from = js_sys::Reflect::get(&obj, &"from".into()).unwrap_throw();
    let to = js_sys::Reflect::get(&obj, &"to".into()).unwrap_throw();
    let src = algebraic_to_bits(JsValue::as_string(&from).unwrap_throw()).unwrap_throw();
    let dst = algebraic_to_bits(JsValue::as_string(&to).unwrap_throw()).unwrap_throw();

    if let Err(e) = game.valid_move(&(src | dst)) {
        return Err(JsError::new(&format!("{}", e)));
    }

    const K: u128 = (0x10 << 0x08) | (0x40 << 0x08);
    const Q: u128 = (0x10 << 0x08) | (0x04 << 0x08);
    #[allow(non_upper_case_globals)]
    const k: u128 = (0x10 << 0x78) | (0x40 << 0x78);
    #[allow(non_upper_case_globals)]
    const q: u128 = (0x10 << 0x78) | (0x04 << 0x78);
    let mv = src | dst;

    if mv == K && game.castling.contains("K") {
        game.move_piece(0xf0 << 0x08);
    }
    else if mv == Q && game.castling.contains("Q") {
        game.move_piece(0x1f << 0x08);
    }
    else if mv == k && game.castling.contains("k") {
        game.move_piece(0xf0 << 0x78);
    }
    else if mv == q && game.castling.contains("q") {
        game.move_piece(0x1f << 0x78);
    }
    else {
        game.move_piece(src | dst)
    }

    let return_fen: String = match fen::encode(&game) {
        Ok(f) => f,
        Err(e) => return Err(JsError::new(&format!("{}", e)))
    };

    Ok(return_fen)
}
