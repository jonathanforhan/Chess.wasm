use std::io::{Result, Error};
use super::pieces::{Color, Piece, Pieces};

pub struct Game {
    pub pieces: Vec<Pieces>,
    pub turn: Color,
    pub castling: String,
    pub en_passant_square: u128,
    pub half_moves: u16,
    pub move_count: u16,
}

impl Game {
    pub fn new(
        pieces: Vec<Pieces>,
        turn: Color,
        castling: String,
        en_passant_square: u128,
        half_moves: u16,
        move_count: u16
    ) -> Self {
        Game { pieces, turn, castling, en_passant_square, half_moves, move_count }
    }

    pub fn moves(&self) -> Vec<Pieces> {
        let mut moves: Vec<Pieces> = Vec::new();

        let mut white_king_moves: Vec<&Pieces> = Vec::new();
        let mut black_king_moves: Vec<&Pieces> = Vec::new();

        let (mut white_attacks, mut black_attacks) = (0u128, 0u128);
        let (mut white_pieces,  mut black_pieces)  = (0u128, 0u128);

        // create boards
        for piece in &self.pieces {
            match piece.color() {
                Color::White => white_pieces |= piece.bits(),
                Color::Black => black_pieces |= piece.bits(),
            }
        }
        // add moves, delay king moves
        for piece in &self.pieces {
            if self.turn == Color::White {
                match (piece, piece.color()) {
                    (Pieces::King(_), Color::White) => {
                        // add white king moves to attacks
                        for m in piece.moves(&black_pieces, &white_pieces) {
                            white_attacks |= m.bits();
                        }
                        // not enough info to process king moves so we save them
                        white_king_moves.push(piece)
                    },
                    (Pieces::Pawn(p), Color::White) => { 
                        moves.append(&mut piece.moves(&black_pieces, &white_pieces));
                        // add pawn attacks to possible checks
                        for a in p.attacks() { white_attacks |= a.bits(); }
                    },
                    (_, Color::White) => { 
                        moves.append(&mut piece.moves(&black_pieces, &white_pieces))
                    },
                    _ => {}
                }
            } else {
                match (piece, piece.color()) {
                    (Pieces::King(_), Color::Black) => {
                        // add black king moves to attacks
                        for m in piece.moves(&white_pieces, &black_pieces) {
                            black_attacks |= m.bits();
                        }
                        black_king_moves.push(piece)
                    },
                    (Pieces::Pawn(p), Color::Black) => { 
                        moves.append(&mut piece.moves(&white_pieces, &black_pieces));
                        for a in p.attacks() { black_attacks |= a.bits(); }
                    },
                    (_, Color::Black) => {
                        moves.append(&mut piece.moves(&white_pieces, &black_pieces))
                    },
                    _ => {}
                }
            }
        }

        // add checks
        for m in &moves {
            match m.color() {
                Color::White => white_attacks |= m.bits(),
                Color::Black => black_attacks |= m.bits(),
            }
        }

        let white_no_go = black_attacks | white_pieces;
        let black_no_go = white_attacks | black_pieces;

        // ensure white does not move into check
        for k in white_king_moves {
            moves.append(&mut k.moves(&black_pieces, &white_no_go));
        }
        for k in black_king_moves {
            moves.append(&mut k.moves(&white_pieces, &black_no_go));
        }

        /* TODO */
        // if castle tag and not check to do it, add castles
        // if enpassant doable, add it
        // find enpassant with <, > because black will always be greater
        // than half of u128_MAX

        return moves;
    }

    pub fn move_piece(&mut self, mv: u128) {
        // create boards
        for piece in &mut self.pieces {
            // move the colors piece
            if *piece.color() == self.turn {
                if piece.bits() & mv != 0 {
                    piece.set_bits(&(piece.bits() ^ mv));
                }
            // subtract a captured piece
            } else {
                if piece.bits() & !mv == 0 {
                    piece.set_bits(&(piece.bits() & mv))
                }
            }
        }
        if let Some(i) = self.pieces.iter().position(|x| *x.bits() == 0) {
            self.pieces.remove(i);
            self.half_moves = 0;
        } else {
            self.half_moves += 1;
        }

        if self.turn == Color::White {
            self.turn = Color::Black
        } else {
            self.move_count += 1;
            self.turn = Color::White;
        }
    }

    pub fn valid_move(&self, mv: &u128) -> Result<()> {
        for m in &self.moves() {
            if m.bits() & mv == *mv {
                return Ok(())
            }
        }
        Err(Error::new(std::io::ErrorKind::Other, "Invalid Move"))
    }
}

