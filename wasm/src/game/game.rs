use std::io::{Result, Error};

use super::{
    pieces::{
        Color,
        Color::White,
        Color::Black,
        Piece,
        Pieces,
        Bishop,
        Knight,
        Rook,
        Queen,
    },
    util::*,
};

#[derive(Clone)]
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

    pub fn debug(&self, white_pieces: &mut u128, black_pieces: &mut u128) {
        for piece in &self.pieces {
            match piece.color() {
                White => *white_pieces |= piece.bits(),
                Black => *black_pieces |= piece.bits(),
            }
        }
    }

    /* Moves are added independent of color using team and opp prefixes
     * essentially treating the color's turn as maximizing or minimizing
     */
    pub fn moves(&self) -> Vec<Pieces> {
        let mut moves: Vec<Pieces> = Vec::new();
        let mut info = GameInfo::init(&self);

        /* Add moves, save king to be evaluated later
         * King moves can only be determined once opposition
         * attacks are known to prevent moving into check
         */
        moves.append(&mut core::gen_moves(&mut info, &self));

        /* Determine king moves using opp_attacks
         * to prevent walking into check
         */
        if info.opp_attacks & info.king.bits() != 0 {
            // if check reset moves
            info.check = true;
            moves.clear();
        }
        moves.append(&mut info.king.moves(&info.opp_attacks, &info.team_pieces));

        /* Adds the castling options if valid
         * a bit is added indicating which option 
         * is chosen for promotion Q, R, B, N
         */
        if !info.check {
            let obstacles = info.opp_attacks | info.opp_pieces | info.team_pieces;
            moves.append(&mut castle::add_castling(&self.castling, &obstacles, self.turn));
        }

        /* trim moves to disgard moving pinned pieces
         * casts feelers from king and detects if peice
         * is checking him
         */
        let mut moves = moves.into_iter().filter(|m| {
            if let Pieces::King(_) = m { return true; }
            check::filter_pins(&info, self.turn, &m.bits())
        }).collect::<Vec<Pieces>>();

        if !info.check { return moves; }

        /* Gen moves that are check-safe
         * does this by casting king to another piece
         * and gen moves. Compare the king's sight
         * to attack piece sight and allows for blocking the check
         */
        moves.append(&mut check::gen_check_moves(&info, &self));

        return moves;
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

        let try_promote = |piece: &mut Pieces, mv: &u128| {
            if mv & (promote::BLACK_BACK_RANK | promote::WHITE_BACK_RANK) == 0 { return; }
            if self.turn == White {
                if mv & promote::WHITE_ROOK != 0 {
                    *piece = Pieces::Rook(Rook::from_bits(*piece.bits(), White));
                }
                else if mv & promote::WHITE_KNIGHT != 0 {
                    *piece = Pieces::Knight(Knight::from_bits(*piece.bits(), White));
                }
                else if mv & promote::WHITE_BISHOP != 0 {
                    *piece = Pieces::Bishop(Bishop::from_bits(*piece.bits(), White));
                }
                else {
                    *piece = Pieces::Queen(Queen::from_bits(*piece.bits(), White));
                }
            } else { // Black
                if mv & promote::BLACK_ROOK != 0 {
                    *piece = Pieces::Rook(Rook::from_bits(*piece.bits(), Black));
                }
                else if mv & promote::BLACK_KNIGHT != 0 {
                    *piece = Pieces::Knight(Knight::from_bits(*piece.bits(), Black));
                }
                else if mv & promote::BLACK_BISHOP != 0 {
                    *piece = Pieces::Bishop(Bishop::from_bits(*piece.bits(), Black));
                }
                else {
                    *piece = Pieces::Queen(Queen::from_bits(*piece.bits(), Black));
                }
            }
        };

        let add_en_passant = |mv: &u128| -> Option<u128> {
            if mv & en_passant::WHITE_EN_PASSANT == *mv {
                let ep = mv & (0xff00 << 0x10);
                return Some(ep << 0x10);
            }
            else if mv & en_passant::BLACK_EN_PASSANT == *mv {
                let ep = mv & (0xff00 << 0x60);
                return Some(ep >> 0x10);
            }
            return None;
        };

        // en passant handlers
        let mut ep_this_turn = false;
        let mut ep_detect = 0; // ensures two pawns moved for ep to be valid
        let mut ep_remove = 64;
        for (i, piece) in self.pieces.iter_mut().enumerate() {
            if *piece.color() == self.turn && piece.bits() & mv != 0 {
                // if pawn move and is en passant, add en passant,
                if let Pieces::Pawn(_) = piece {
                    ep_detect += 1;
                    if let Some(ep) = add_en_passant(&mv) {
                        self.en_passant_square = ep;
                        ep_this_turn = true;
                    }
                    try_promote(piece, &mv);
                }

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
                if self.en_passant_square & mv == 0 { continue; }
                if *piece.color() == White {
                    if piece.bits() & (self.en_passant_square << 0x10) != 0 {
                        ep_detect += 1;
                        ep_remove = i;
                    }
                } else { // Black
                    if piece.bits() & (self.en_passant_square >> 0x10) != 0 {
                        ep_detect += 1;
                        ep_remove = i;
                    }
                }
            }
        } // end for loop

        // if en passant was not added this turn reset it
        if !ep_this_turn { self.en_passant_square = 0u128; }
        // if enpassant, remove taken piece
        if ep_detect == 2 { removed.push(ep_remove); }

        // remove any captured or promoted pieces
        for i in &removed { self.pieces.remove(*i); }
        // adjust half moves to reflect turn
        if removed.len() > 0 { self.half_moves = 0; } else { self.half_moves += 1; }

        if self.turn == White {
            self.turn = Black
        } else {
            self.move_count += 1;
            self.turn = White;
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

