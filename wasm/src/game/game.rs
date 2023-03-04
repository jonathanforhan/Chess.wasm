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
        self.init_boards(&mut white_pieces, &mut black_pieces);

        /* Add moves, save king moves to be evaluated later
         * King moves can only be determined once opposition
         * attacks are known to prevent moving into check
         */
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
            } else { // black move
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
            } // endif
        } // end for-loop

        /* Can now determine king moves and castling
         * castling moves are wrapped in king struct
         */
        match &self.turn {
            Color::White => {
                // ensure white does not move into check
                for k in king_moves {
                    moves.append(&mut k.moves(&black_attacks, &white_pieces));
                }
                if self.castling.contains('K') {
                    if castle::K_VALID & (black_attacks | black_pieces | white_pieces) == 0 {
                        moves.push(Pieces::King(King::from_bits(castle::K_ZONE, Color::White)));
                    }
                }
                if self.castling.contains('Q') {
                    if castle::Q_VALID & (black_attacks | black_pieces | white_pieces) == 0 {
                        moves.push(Pieces::King(King::from_bits(castle::Q_ZONE, Color::White)));
                    }
                }
            },
            Color::Black => {
                for k in king_moves {
                    moves.append(&mut k.moves(&white_attacks, &black_pieces));
                }
                if self.castling.contains('k') {
                    if castle::k_VALID & (white_attacks | white_pieces | black_pieces) == 0 {
                        moves.push(Pieces::King(King::from_bits(castle::k_ZONE, Color::Black)));
                    }
                }
                if self.castling.contains('q') {
                    if castle::q_VALID & (white_attacks | white_pieces | black_pieces) == 0 {
                        moves.push(Pieces::King(King::from_bits(castle::q_ZONE, Color::Black)));
                    }
                }
            },
        } // end match

        /* TODO */
        // if enpassant doable, add it
        // find enpassant with <, > because black will always be greater
        // than half of u128_MAX
        // add pawn promotion
        // Check detection

        return moves;
    }

    pub fn init_boards(&self, white_pieces: &mut u128, black_pieces: &mut u128) {
        for piece in &self.pieces {
            match piece.color() {
                Color::White => *white_pieces |= piece.bits(),
                Color::Black => *black_pieces |= piece.bits(),
            }
        }
    }

    pub fn move_piece(&mut self, mv: u128) {
        let mut removed: Vec::<usize> = Vec::new();

        let try_castle = |piece: &mut Pieces, king_move: u128, rook_move: u128| {
            if let Pieces::King(k) = piece {
                k.set_bits(&(k.bits() ^ king_move));
            } else if let Pieces::Rook(r) = piece {
                r.set_bits(&(r.bits() ^ rook_move));
            }
        };

        let fix_castle = |mut s: String, c: char| {
            s.remove(s.find(c).unwrap());
            if s.len() == 0 { s = String::from("-"); }
            return s;
        };

        let check_en_passant = |mv: &u128| -> Option<u128> {
            const WHITE_EN_PASSANT: u128 = 0xff00 << 0x10 | 0xff00 << 0x30;
            const BLACK_EN_PASSANT: u128 = 0xff00 << 0x60 | 0xff00 << 0x40;
            
            match self.turn {
                Color::White => {
                    if mv & WHITE_EN_PASSANT == *mv {
                        let en_passant = mv & (0xff00 << 0x10);
                        return Some(en_passant << 0x10);
                    }
                },
                Color::Black => {
                    if mv & BLACK_EN_PASSANT == *mv {
                        let en_passant = mv & (0xff00 << 0x60);
                        return Some(en_passant >> 0x10);
                    }
                },
            }
            return None;
        };

        self.en_passant_square = 0x0; // reset
        for (i, piece) in self.pieces.iter_mut().enumerate() {
            // move the colors piece
            if *piece.color() == self.turn && piece.bits() & mv != 0 {
                // if pawn move and is en passant, add en passant
                // execute move and return
                if let Pieces::Pawn(_) = piece {
                    if let Some(ep) = check_en_passant(&mv) {
                        self.en_passant_square = ep;
                    }
                };

                match mv {
                    castle::K_ZONE => try_castle(piece, castle::K_MOVE, castle::K_ROOK),
                    castle::Q_ZONE => try_castle(piece, castle::Q_MOVE, castle::Q_ROOK),
                    castle::k_ZONE => try_castle(piece, castle::k_MOVE, castle::k_ROOK),
                    castle::q_ZONE => try_castle(piece, castle::q_MOVE, castle::q_ROOK),
                    _ => piece.set_bits(&(piece.bits() ^ mv))
                };

                if self.castling == "-" { continue; } // guard pointless checks
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
            // add empty boards to trash list
            // added by index for easy removal
            else if piece.bits() & !mv == 0 {
                removed.push(i);
            }
        } // end for loop

        // remove any captured or promoted pieces
        for i in &removed { self.pieces.remove(*i); }
        // adjust half moves to reflect turn
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

