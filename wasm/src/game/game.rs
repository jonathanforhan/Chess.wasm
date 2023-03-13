use std::io::{Result, Error};
use super::{
    pieces::{
        Color,
        Color::White,
        Color::Black,
        Piece,
        Pieces,
    },
    util::*,
};

#[derive(Clone)]
pub struct Game {
    pub pieces: Vec<Pieces>,
    pub turn: Color,
    pub castling: u16,
    pub en_passant_square: u128,
    pub half_moves: u16,
    pub move_count: u16,
}

impl Game {
    pub fn new(
        pieces: Vec<Pieces>,
        turn: Color,
        castling: u16,
        en_passant_square: u128,
        half_moves: u16,
        move_count: u16
    ) -> Self {
        Game { pieces, turn, castling, en_passant_square, half_moves, move_count }
    }

    pub fn moves(&self) -> Vec<Pieces> {
        self.moves_verbose().0
    }

    pub fn info(&self) -> GameInfo {
        // high overhead, prefer moves_verbose
        // in almost all situations
        self.moves_verbose().1
    }

    /* Moves are added independent of color using team and opp prefixes
     * essentially treating the color's turn as maximizing or minimizing
     */
    pub fn moves_verbose(&self) -> (Vec<Pieces>, GameInfo) {
        let mut moves: Vec<Pieces> = Vec::with_capacity(64);
        let mut info = GameInfo::init(&self);

        /* Add moves, save king to be evaluated later
         * King moves can only be determined once opposition
         * attacks are known to prevent moving into check
         */
        core::gen_moves(&mut info, &self, &mut moves);

        /* Determine king moves using opp_attacks
         * to prevent walking into check
         */
        if info.opp_attacks & info.king.bits() != 0 {
            info.check = true;
            moves.clear();
        }
        info.king.moves(&info.opp_attacks, &info.team_pieces, &mut moves);

        /* Adds the castling options if valid
         * a bit is added indicating which option 
         * is chosen for promotion Q, R, B, N
         */
        if !info.check {
            let obstacles = (info.opp_attacks & !castle::EDGE_CASE) | info.opp_pieces | info.team_pieces;
            castle::add_castling(self.castling, &obstacles, self.turn, &mut moves);
        }

        /* trim moves to disgard moving pinned pieces
         * casts feelers from king and detects if peice
         * is checking him
         */
        if !info.check {
            let moves = moves.into_iter().filter(|m| {
                if let Pieces::King(_) = *m { return true; }
                check::filter_pins(&info, &self, &m.bits())
            }).collect::<Vec<Pieces>>();
            info.valid_moves = moves.len() as u16;
            return (moves, info);
        }

        /* Gen moves that are check-safe
         * does this by casting king to another piece
         * and gen moves. Compare the king's sight
         * to attack piece sight and allows for blocking the check
         */
        check::gen_check_moves(&info, &self, &mut moves);

        let moves = moves.into_iter().filter(|m| {
            if let Pieces::King(_) = *m { return true; }
            check::filter_pins(&info, &self, &m.bits())
        }).collect::<Vec<Pieces>>();
        info.valid_moves = moves.len() as u16;
        return (moves, info);
    }

    pub fn move_piece(&mut self, mv: u128) {
        let mut remove: Option<usize> = None;
        let mut ep_remove: Option<usize> = None;

        let mut ep_this_turn = false;
        let mut two_pawn_moves = (false, false);
        for (i, piece) in self.pieces.iter_mut().enumerate() {
            if *piece.color() == self.turn && piece.bits() & mv != 0 {
                // if pawn move and is en passant, add en passant,
                if let Pieces::Pawn(_) = piece {
                    two_pawn_moves.0 = true;
                    if let Some(ep) = en_passant::add_en_passant(&mv) {
                        self.en_passant_square = ep;
                        ep_this_turn = true;
                    }
                    promote::try_promote(piece, &mv, self.turn);
                }

                if self.castling == 0 {
                    piece.set_bits(&(piece.bits() ^ mv));
                    continue;
                }

                match mv {
                    castle::K_ZONE => castle::try_castle(piece, castle::K_MOVE, castle::K_ROOK),
                    castle::Q_ZONE => castle::try_castle(piece, castle::Q_MOVE, castle::Q_ROOK),
                    castle::k_ZONE => castle::try_castle(piece, castle::k_MOVE, castle::k_ROOK),
                    castle::q_ZONE => castle::try_castle(piece, castle::q_MOVE, castle::q_ROOK),
                    _ => piece.set_bits(&(piece.bits() ^ mv))
                };

                self.castling = castle::fix_castle(self.castling, &mv);
            }
            else if piece.bits() & !mv == 0 {
                remove = Some(i);
            }
            else if let Pieces::Pawn(p) = piece {
                if self.en_passant_square & mv == 0 { continue; }
                let en_passant_square: u128;
                match p.color() {
                    White => en_passant_square = self.en_passant_square << 0x10,
                    Black => en_passant_square = self.en_passant_square >> 0x10,
                }
                if p.bits() & en_passant_square != 0 {
                    two_pawn_moves.1 = true;
                    ep_remove = Some(i);
                }
            }
        } // end for loop

        if !ep_this_turn { self.en_passant_square = 0u128; }
        if two_pawn_moves == (true, true) { remove = ep_remove; }

        if let Some(i) = remove {
            self.pieces.remove(i);
            self.half_moves = 0;
        } else {
            self.half_moves += 1;
        }

        match self.turn {
            White => self.turn = Black,
            Black => {
                self.move_count += 1;
                self.turn = White;
            }
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

