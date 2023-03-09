use wasm_bindgen::prelude::*;
use js_sys;

use crate::game::{
    util::*,
    Game,
    pieces::Color,
    pieces::Piece,
    pieces::Pieces,
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
    let mut current = 0u128;
    for p in &game.pieces {
        if p.color() == &game.turn { current |= p.bits(); }
    }

    let arr = js_sys::Array::new();

    let mvs = game.moves();
    for m in mvs {
        let src = current & m.bits();  // find the matching starting location
        let mut dst = m.bits() & !src; // subtract starting pos from move map
        let mut promotion = 0u128;
        if let Pieces::Pawn(_) = m {
            match m.color() {
                Color::White => dst &= !promote::BLACK_BACK_RANK,
                Color::Black => dst &= !promote::WHITE_BACK_RANK,
            }
            promotion |= m.bits() ^ (src | dst);
        }

        let promotion = match promotion {
            promote::WHITE_ROOK => "W",
            promote::WHITE_BISHOP => "B",
            promote::WHITE_KNIGHT => "N",
            promote::BLACK_ROOK => "w",
            promote::BLACK_BISHOP => "b",
            promote::BLACK_KNIGHT => "n",
            _ => "",
        };
        /* TODO */
        // opponent castling javascript wrapper

        // Convert bits to string
        let (from, to) = (bits_to_algebraic(&src)?, bits_to_algebraic(&dst)?);
        let obj = js_sys::Object::new();

        // Wrap in JS object
        js_sys::Reflect::set(&obj, &"from".into(), &JsValue::from_str(&from))
            .map_err(|_| JsError::new("Wasm object access error"))?;
        js_sys::Reflect::set(&obj, &"to".into(), &JsValue::from_str(&to))
            .map_err(|_| JsError::new("Wasm object access error"))?;
        js_sys::Reflect::set(&obj, &"promotion".into(), &JsValue::from_str(&promotion))
            .map_err(|_| JsError::new("Wasm object access error"))?;

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

    // Unwrap JS object to rust data-type
    let from = js_sys::Reflect::get(&obj, &"from".into())
        .map_err(|_| JsError::new("Wasm object access error"))?;

    let to = js_sys::Reflect::get(&obj, &"to".into())
        .map_err(|_| JsError::new("Wasm object access error"))?;

    let promotion = js_sys::Reflect::get(&obj, &"promotion".into())
        .map_err(|_| JsError::new("Wasm object access error"))?;

    let src = algebraic_to_bits(JsValue::as_string(&from)
        .ok_or_else(|| JsError::new("Move parse error"))?)?;

    let dst = algebraic_to_bits(JsValue::as_string(&to)
        .ok_or_else(|| JsError::new("Move parse error"))?)?;

    let promotion = JsValue::as_string(&promotion)
        .ok_or_else(|| JsError::new("Wasm object access error"))?;

    let mut mv = src | dst;

    // Check castle move
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

    // Check pawn promotion
    match &promotion as &str {
        "R" => mv |= promote::WHITE_ROOK,
        "N" => mv |= promote::WHITE_KNIGHT,
        "B" => mv |= promote::WHITE_BISHOP,
        "r" => mv |= promote::BLACK_ROOK,
        "n" => mv |= promote::BLACK_KNIGHT,
        "b" => mv |= promote::BLACK_BISHOP,
        "" => (),
        _ => { return Err(JsError::new("Invalid promotion")); }
    }

    // Validate move
    if let Err(e) = game.valid_move(&(mv)) {
        return Err(JsError::new(&format!("{}", e)));
    }

    // Execute move
    game.move_piece(mv);

    // Return new fen
    let return_fen: String = match fen::encode(&game) {
        Ok(f) => f,
        Err(e) => return Err(JsError::new(&format!("{}", e)))
    };

    Ok(return_fen)
}
