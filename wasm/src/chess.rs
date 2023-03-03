use wasm_bindgen::prelude::*;
use js_sys;

use crate::game::castle;

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

    let from = js_sys::Reflect::get(&obj, &"from".into())
        .map_err(|_| JsError::new("Wasm object access error"))?;

    let to = js_sys::Reflect::get(&obj, &"to".into())
        .map_err(|_| JsError::new("Wasm object access error"))?;

    let src = algebraic_to_bits(JsValue::as_string(&from)
        .ok_or_else(|| JsError::new("Move parse error"))?)?;

    let dst = algebraic_to_bits(JsValue::as_string(&to)
        .ok_or_else(|| JsError::new("Move parse error"))?)?;

    let mut mv = src | dst;

    if mv == castle::K_MOVE && game.castling.contains("K") {
        mv = castle::K_ZONE;
    }
    else if mv == castle::Q_MOVE && game.castling.contains("Q") {
        mv = castle::Q_ZONE;
    }
    else if mv == castle::k_MOVE && game.castling.contains("k") {
        mv = castle::k_ZONE;
    }
    else if mv == castle::q_MOVE && game.castling.contains("q") {
        mv = castle::q_ZONE;
    }

    if let Err(e) = game.valid_move(&(mv)) {
        return Err(JsError::new(&format!("{}", e)));
    }

    game.move_piece(mv);

    let return_fen: String = match fen::encode(&game) {
        Ok(f) => f,
        Err(e) => return Err(JsError::new(&format!("{}", e)))
    };

    Ok(return_fen)
}
