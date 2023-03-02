use std::io::{Result, Error};

use super::{
    pieces::{
        Color,
        Piece,
        Pieces,
        King
    },
    castle,
};

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
                if self.castling.contains('K') {
                    const CASTLE: u128 = 0xf0 << 0x08; // K_ZONE now @refactor
                    const VALID: u128 = 0x60 << 0x08;
                    if VALID & (black_attacks | black_pieces | white_pieces) == 0 {
                        moves.push(Pieces::King(King::from_bits(CASTLE, Color::White)));
                    }
                }
                if self.castling.contains('Q') {
                    const CASTLE: u128 = 0x1f << 0x08;
                    const VALID: u128 = 0x0e << 0x08;
                    if VALID & (black_attacks | black_pieces | white_pieces) == 0 {
                        moves.push(Pieces::King(King::from_bits(CASTLE, Color::White)));
                    }
                }
            },
            Color::Black => {
                for k in king_moves {
                    moves.append(&mut k.moves(&white_attacks, &black_pieces));
                }
                if self.castling.contains('k') {
                    #[allow(non_upper_case_globals)]
                    const CASTLE: u128 = 0xf0 << 0x78;
                    const VALID: u128 = 0x60 << 0x78;
                    if VALID & (white_attacks | white_pieces | black_pieces) == 0 {
                        moves.push(Pieces::King(King::from_bits(CASTLE, Color::Black)));
                    }
                }
                if self.castling.contains('q') {
                    #[allow(non_upper_case_globals)]
                    const CASTLE: u128 = 0x1f << 0x78;
                    const VALID: u128 = 0x0e << 0x78;
                    if VALID & (white_attacks | white_pieces | black_pieces) == 0 {
                        moves.push(Pieces::King(King::from_bits(CASTLE, Color::Black)));
                    }
                }
            },
        }

        /* TODO */
        // @Refactor! getting a little spaghettified
        // if enpassant doable, add it
        // find enpassant with <, > because black will always be greater
        // than half of u128_MAX
        // add pawn promotion
        // Check detection

        return moves;
    }

    //fn gen_attacks() -> Vec<Pieces> {
        //
    //}
//
    //fn gen_castle() -> Vec<Pieces> {
        //
    //}
//
    //fn gen_en_passant() -> Vec<Pieces> {
        //
    //}

    pub fn move_piece(&mut self, mv: u128) {
        let mut removed: Vec::<usize> = Vec::new();
        // create boards
        for (i, piece) in self.pieces.iter_mut().enumerate() {
            // move the colors piece
            if *piece.color() == self.turn {
                if piece.bits() & mv != 0 {
                    #[allow(non_upper_case_globals)]
                    match mv {
                        castle::K_ZONE => {
                            if let Pieces::King(k) = piece {
                                k.set_bits(&(k.bits() ^ castle::K_MOVE));
                            } else if let Pieces::Rook(r) = piece {
                                const ROOK_MOVE: u128 = 0xa0 << 0x08;
                                r.set_bits(&(r.bits() ^ ROOK_MOVE));
                            }
                        },
                        castle::Q_ZONE => {
                            if let Pieces::King(k) = piece {
                                k.set_bits(&(k.bits() ^ castle::Q_MOVE));
                            } else if let Pieces::Rook(r) = piece {
                                const ROOK_MOVE: u128 = 0x09 << 0x08;
                                r.set_bits(&(r.bits() ^ ROOK_MOVE));
                            }
                        },
                        castle::k_ZONE => {
                            if let Pieces::King(k) = piece {
                                k.set_bits(&(k.bits() ^ castle::k_MOVE));
                            } else if let Pieces::Rook(r) = piece {
                                const ROOK_MOVE: u128 = 0xa0 << 0x78;
                                r.set_bits(&(r.bits() ^ ROOK_MOVE));
                            }
                        },
                        castle::q_ZONE => {
                            if let Pieces::King(k) = piece {
                                k.set_bits(&(k.bits() ^ castle::q_MOVE));
                            } else if let Pieces::Rook(r) = piece {
                                const ROOK_MOVE: u128 = 0x09 << 0x78;
                                r.set_bits(&(r.bits() ^ ROOK_MOVE));
                            }
                        },
                        _ => piece.set_bits(&(piece.bits() ^ mv))
                    };

                    let fix_castle = |mut s: String, c: char| {
                        s.remove(s.find(c).unwrap());
                        if s.len() == 0 { s = String::from("-"); }
                        return s;
                    };
                    // adjust castling rights
                    if self.castling.contains('K') && mv & castle::K_SQUARES != 0 {
                        self.castling = fix_castle(self.castling.clone(), 'K');
                    }
                    if self.castling.contains('Q') && mv & castle::Q_SQUARES != 0 {
                        self.castling = fix_castle(self.castling.clone(), 'Q');
                    }
                    if self.castling.contains('k') && mv & castle::k_SQUARES != 0 {
                        self.castling = fix_castle(self.castling.clone(), 'k');
                    }
                    if self.castling.contains('q') && mv & castle::q_SQUARES != 0 {
                        self.castling = fix_castle(self.castling.clone(), 'q');
                    }
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
            if m.bits() ^ mv == 0 {
                return Ok(())
            }
        }
        Err(Error::new(std::io::ErrorKind::Other, "Invalid Move"))
    }
}

