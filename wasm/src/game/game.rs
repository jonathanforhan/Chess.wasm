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

        // throw away values
        let mut king: &Pieces = &Pieces::King(King::from_bits(0x0001, Color::White));

        let (mut white_attacks, mut black_attacks): (u128, u128) = (0, 0);
        let (mut white_pieces,  mut black_pieces):  (u128, u128) = (0, 0);
        let mut check = false;

        self.init_boards(&mut white_pieces, &mut black_pieces);

        /* Add moves, save king moves to be evaluated later
         * King moves can only be determined once opposition
         * attacks are known to prevent moving into check
         */
        for piece in &self.pieces {
            if self.turn == Color::White {
                match (piece, piece.color()) {
                    (Pieces::King(_), Color::White) => {
                        king = piece;
                    },
                    (Pieces::Pawn(_), Color:: White) => {
                        moves.append(&mut piece.moves(&(black_pieces | self.en_passant_square), &white_pieces));
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
                        king = piece;
                    },
                    (Pieces::Pawn(_), Color:: Black) => {
                        moves.append(&mut piece.moves(&(white_pieces | self.en_passant_square), &black_pieces));
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

        /* Determine king moves and castling
         * castling moves are wrapped in king struct
         * if check on king break loop and report only the king moves to escape
         */
        match &self.turn {
            Color::White => {
                // if check return only valid kingmoves
                if black_attacks & king.bits()  != 0 {
                    return king.moves(&black_attacks, &white_pieces);
                }
                // comparing against black attacks allows to not move into check
                moves.append(&mut king.moves(&black_attacks, &white_pieces));

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
                if white_attacks & king.bits()  != 0 {
                    return king.moves(&white_attacks, &black_pieces);
                }
                moves.append(&mut king.moves(&white_attacks, &black_pieces));

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

        let add_en_passant = |mv: &u128| -> Option<u128> {
            const WHITE_EN_PASSANT: u128 = 0xff00 << 0x10 | 0xff00 << 0x30;
            const BLACK_EN_PASSANT: u128 = 0xff00 << 0x60 | 0xff00 << 0x40;

            if mv & WHITE_EN_PASSANT == *mv {
                let en_passant = mv & (0xff00 << 0x10);
                return Some(en_passant << 0x10);
            }
            else if mv & BLACK_EN_PASSANT == *mv {
                let en_passant = mv & (0xff00 << 0x60);
                return Some(en_passant >> 0x10);
            }
            return None;
        };

        // en passant handlers
        let mut ep_this_turn = false;
        let mut ep_detect = 0; // ensures two pawns moved for ep to be valid
        let mut ep_remove: usize = 64;
        for (i, piece) in self.pieces.iter_mut().enumerate() {
            // move the colors piece
            if *piece.color() == self.turn && piece.bits() & mv != 0 {
                // if pawn move and is en passant, add en passant,
                if let Pieces::Pawn(_) = piece {
                    ep_detect += 1;
                    // add en passant
                    if let Some(ep) = add_en_passant(&mv) {
                        self.en_passant_square = ep;
                        ep_this_turn = true;
                    // attack enemy en apssant
                    }
                } // end if-let

                match mv {
                    castle::K_ZONE => try_castle(piece, castle::K_MOVE, castle::K_ROOK),
                    castle::Q_ZONE => try_castle(piece, castle::Q_MOVE, castle::Q_ROOK),
                    castle::k_ZONE => try_castle(piece, castle::k_MOVE, castle::k_ROOK),
                    castle::q_ZONE => try_castle(piece, castle::q_MOVE, castle::q_ROOK),
                    _ => piece.set_bits(&(piece.bits() ^ mv))
                };

                // guard pointless checks
                if self.castling == "-" { continue; }
                // fix castle rights
                let mut fix_castle = |c: char, castle: u128| {
                    if self.castling.contains(c) && mv & castle != 0 {
                        let mut s = self.castling.clone();
                        s.remove(s.find(c).unwrap());
                        if s.len() == 0 { s = String::from("-"); }
                        self.castling = s;
                    }
                };
                fix_castle('K', castle::K_SQUARES);
                fix_castle('Q', castle::Q_SQUARES);
                fix_castle('k', castle::k_SQUARES);
                fix_castle('q', castle::q_SQUARES);
                continue;
            }
            // add empty boards to trash list
            // added by index for easy removal
            else if piece.bits() & !mv == 0 {
                removed.push(i);
            }
            else if let Pieces::Pawn(_) = piece {
                if !(self.en_passant_square & mv != 0) { continue; }
                match piece.color() {
                    Color::White => {
                        if piece.bits() & (self.en_passant_square << 0x10) != 0 {
                            ep_detect += 1;
                            ep_remove = i;
                        }
                    },
                    Color::Black => {
                        if piece.bits() & (self.en_passant_square >> 0x10) != 0 {
                            ep_detect += 1;
                            ep_remove = i;
                        }
                    },
                }
            }
        } // end for loop

        // if en passant was not added this turn reset it
        if !ep_this_turn { self.en_passant_square = 0x0; }
        // if enpassant, remove taken piece
        if ep_detect == 2 { removed.push(ep_remove); }

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

