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

        let mut king_moves: Vec<&Pieces> = Vec::new();
        let (mut white_attacks, mut black_attacks): (u128, u128) = (0, 0);
        let (mut white_pieces,  mut black_pieces):  (u128, u128) = (0, 0);



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
                        // not enough info to process king moves so we save them
                        king_moves.push(piece)
                    },
                    (_, Color::White) => { 
                        moves.append(&mut piece.moves(&black_pieces, &white_pieces))
                    },
                    (Pieces::Pawn(p), Color::Black) => { 
                        for a in p.attacks() { black_attacks |= a.bits(); }
                    },
                    (_, Color::Black) => {
                        // must add own pieces to opp to detect check
                        for m in piece.moves(&(white_pieces | black_pieces), &0u128) {
                            black_attacks |= m.bits() ^ piece.bits();
                        }
                    }
                }
            } else {
                match (piece, piece.color()) {
                    (Pieces::King(_), Color::Black) => {
                        king_moves.push(piece)
                    },
                    (_, Color::Black) => {
                        moves.append(&mut piece.moves(&white_pieces, &black_pieces))
                    },
                    (Pieces::Pawn(p), Color::White) => { 
                        for a in p.attacks() { white_attacks |= a.bits(); }
                    },
                    (_, Color::White) => {
                        for m in piece.moves(&(black_pieces | white_pieces), &0u128) {
                            white_attacks |= m.bits() ^ piece.bits();
                        }
                    }
                }
            }
        }

        match &self.turn {
            Color::White => {
                // ensure white does not move into check
                for k in king_moves {
                    moves.append(&mut k.moves(&black_attacks, &white_pieces));
                }
            },
            Color::Black => {
                for k in king_moves {
                    moves.append(&mut k.moves(&white_attacks, &black_pieces));
                }
            },
        }



        /* TODO */
        // if castle tag and not check to do it, add castles
        // if enpassant doable, add it
        // find enpassant with <, > because black will always be greater
        // than half of u128_MAX
        // add pawn promotion

        return moves;
    }

    pub fn move_piece(&mut self, mv: u128) {
        let mut removed: Vec::<usize> = Vec::new();
        // create boards
        for (i, piece) in self.pieces.iter_mut().enumerate() {
            // move the colors piece
            if *piece.color() == self.turn {
                if piece.bits() & mv != 0 {
                    piece.set_bits(&(piece.bits() ^ mv));
                }
            // set captured piece to zero
            } else {
                if piece.bits() & !mv == 0 {
                    removed.push(i);
                }
            }
        }

        for i in &removed { self.pieces.remove(*i); } // remove any captured or promoted pieces
        if removed.len() > 0 { self.half_moves = 0; } else { self.half_moves += 1; }

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

