use wasm_bindgen::UnwrapThrowExt;

use crate::{BitBoard, Color};

pub struct Game {
    pub pieces: Vec<BitBoard>,
    pub turn: Color,
    pub castling: String,
    pub en_passant_square: BitBoard,
    pub half_moves: u16,
    pub move_count: u16,
}

impl Game {
    pub fn new(
        pieces: Vec<BitBoard>,
        turn: Color,
        castling: String,
        en_passant_square: BitBoard,
        half_moves: u16,
        move_count: u16
    ) -> Self {
        Game { pieces, turn, castling, en_passant_square, half_moves, move_count }
    }

    pub fn moves(&self) -> Vec<BitBoard> {
        let mut moves: Vec<BitBoard> = Vec::new();

        let mut white_king_moves: Vec<&BitBoard> = Vec::new();
        let mut black_king_moves: Vec<&BitBoard> = Vec::new();

        let mut white_attacks: u128 = 0;
        let mut black_attacks: u128 = 0;

        let mut white_pieces: u128 = 0;
        let mut black_pieces: u128 = 0;

        // create boards
        for piece in &self.pieces {
            match piece.color().unwrap_throw() {
                Color::White => white_pieces |= piece.bits().unwrap_throw(),
                Color::Black => black_pieces |= piece.bits().unwrap_throw(),
            }
        }
        // add moves, delay king moves
        for piece in &self.pieces {
            match (piece, piece.color().unwrap_throw()) {
                (BitBoard::King(_), Color::White) => {
                    // add white king moves to attacks
                    let _ = piece.moves(&black_pieces, &white_pieces).unwrap_throw()
                        .iter().map(|x| white_attacks |= x.bits().unwrap_throw());
                    // not enough info to process king moves so we save them
                    white_king_moves.push(piece)
                },
                (_, Color::White) => { 
                    moves.append(&mut piece.moves(&black_pieces, &white_pieces).unwrap_throw())
                },
                (BitBoard::King(_), Color::Black) => {
                    // add black king moves to attacks
                    let _ = piece.moves(&white_pieces, &black_pieces).unwrap_throw()
                        .iter().map(|x| black_attacks |= x.bits().unwrap_throw());
                    black_king_moves.push(piece)
                },
                (_, Color::Black) => {
                    moves.append(&mut piece.moves(&white_pieces, &black_pieces).unwrap_throw())
                },
            }
        }

        // add checks
        for m in &moves {
            match m.color().unwrap_throw() {
                Color::White => white_attacks |= m.bits().unwrap_throw(),
                Color::Black => black_attacks |= m.bits().unwrap_throw(),
            }
        }

        let white_no_go = black_attacks | white_pieces;
        let black_no_go = white_attacks | black_pieces;

        // ensure white does not move into check
        for k in white_king_moves {
            moves.append(&mut k.moves(&black_pieces, &white_no_go).unwrap_throw());
        }
        for k in black_king_moves {
            moves.append(&mut k.moves(&white_pieces, &black_no_go).unwrap_throw());
        }

        /* TODO */
        // add pawn attacks as checks
        // if castle tag and not check to do it, add castles
        // if enpassant doable, add it

        moves
    }
}
