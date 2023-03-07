use std::io::{Result, Error};

use super::{
    pieces::{
        Color,
        Color::White,
        Color::Black,
        Piece,
        Pieces,
        Pawn,
        Bishop,
        Knight,
        Rook,
        Queen,
        King,
    },
    util::*,
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
        
        let mut king: Option<&Pieces> = None; // not really optional
        let mut check = false;
        let (mut white_attacks, mut black_attacks): (u128, u128) = (0x0, 0x0);
        let (mut white_pieces,  mut black_pieces):  (u128, u128) = (0x0, 0x0);
        let mut opp_bishop: u128 = 0x0;
        let mut opp_rook: u128 = 0x0;
        let mut opp_queen: u128 = 0x0;

        self.init_boards(&mut white_pieces, &mut black_pieces);

        let add_white_promotion = |mv: &Pieces, mvs: &mut Vec<Pieces>| {
            mvs.push(Pieces::Pawn(Pawn::from_bits(mv.bits() | promote::WHITE_ROOK, White)));
            mvs.push(Pieces::Pawn(Pawn::from_bits(mv.bits() | promote::WHITE_KNIGHT, White)));
            mvs.push(Pieces::Pawn(Pawn::from_bits(mv.bits() | promote::WHITE_BISHOP, White)));
        };

        let add_black_promotion = |mv: &Pieces, mvs: &mut Vec<Pieces>| {
            mvs.push(Pieces::Pawn(Pawn::from_bits(mv.bits() | promote::BLACK_ROOK, Black)));
            mvs.push(Pieces::Pawn(Pawn::from_bits(mv.bits() | promote::BLACK_KNIGHT, Black)));
            mvs.push(Pieces::Pawn(Pawn::from_bits(mv.bits() | promote::BLACK_BISHOP, Black)));
        };

        // removes moves that cause 
        let filter_pins = |king_pos: &u128, mv: &u128, test_diagonal: &u128, test_straight: &u128| -> bool {
            let test_pieces = match self.turn {
                White => white_pieces ^ mv,
                Black => black_pieces ^ mv
            };

            let diagonal = Pieces::Bishop(Bishop::from_bits(*king_pos, self.turn));
            let straight = Pieces::Rook(Rook::from_bits(*king_pos, self.turn));
            
            for m in diagonal.moves(test_diagonal, &test_pieces) {
                if m.bits() & test_diagonal != 0 { return false; }
            }
            for m in straight.moves(test_straight, &test_pieces) {
                if m.bits() & test_straight != 0 { return false; }
            }
            return true;
        };

        /* Add moves, save king moves to be evaluated later
         * King moves can only be determined once opposition
         * attacks are known to prevent moving into check
         */
        for piece in &self.pieces {
            match (self.turn, piece.color(), piece) {
                // White turn
                (White, White, Pieces::King(_)) => {
                    king = Some(piece);
                },
                (White, White, Pieces::Pawn(_)) => {
                    let mut mvs = piece.moves(&(black_pieces | self.en_passant_square), &white_pieces);
                    for mv in &mvs {
                        if mv.bits() & promote::WHITE_BACK_RANK != 0 {
                            add_white_promotion(mv, &mut moves);
                        }
                    }
                    moves.append(&mut mvs);
                },
                (White, White, _) => {
                    moves.append(&mut piece.moves(&black_pieces, &white_pieces))
                },
                (White, Black, Pieces::Pawn(p)) => {
                    for a in p.attacks() { black_attacks |= a.bits(); }
                },
                (White, Black, _) => {
                    // must add own pieces to opp to detect check
                    for m in piece.moves(&(white_pieces | black_pieces), &0u128) {
                        black_attacks |= m.bits() ^ piece.bits();
                        match m {
                            Pieces::Bishop(_) => opp_bishop |= piece.bits(),
                            Pieces::Rook(_) => opp_rook |= piece.bits(),
                            Pieces::Queen(_) => opp_queen |= piece.bits(),
                            _ => ()
                        }
                    }
                },
                // Black turn
                (Black, Black, Pieces::King(_)) => {
                    king = Some(piece);
                },
                (Black, Black, Pieces::Pawn(_)) => {
                    let mut mvs = piece.moves(&(white_pieces | self.en_passant_square), &black_pieces);
                    for mv in &mvs {
                        if mv.bits() & promote::BLACK_BACK_RANK != 0 {
                            add_black_promotion(mv, &mut moves);
                        }
                    }
                    moves.append(&mut mvs);
                },
                (Black, Black, _) => {
                    moves.append(&mut piece.moves(&white_pieces, &black_pieces))
                },
                (Black, White, Pieces::Pawn(p)) => {
                    for a in p.attacks() { white_attacks |= a.bits(); }
                },
                (Black, White, _) => {
                    for m in piece.moves(&(black_pieces | white_pieces), &0u128) {
                        white_attacks |= m.bits() ^ piece.bits();
                        match m {
                            Pieces::Bishop(_) => opp_bishop |= piece.bits(),
                            Pieces::Rook(_) => opp_rook |= piece.bits(),
                            Pieces::Queen(_) => opp_queen |= piece.bits(),
                            _ => ()
                        }
                    }
                },
            }
        } // end for-loop
        
        /* Determine king moves and castling
         * castling moves are wrapped in king struct
         */
        let king = king.unwrap();
        if self.turn == White {
            // if check return only valid kingmoves
            if black_attacks & king.bits() != 0 {
                check = true;
                moves.clear();
            }
            // comparing against black attacks allows to not move into check
            moves.append(&mut king.moves(&black_attacks, &white_pieces));

            if !check && self.castling.contains('K') {
                if castle::K_VALID & (black_attacks | black_pieces | white_pieces) == 0 {
                    moves.push(Pieces::King(King::from_bits(castle::K_ZONE, White)));
                }
            }
            if !check && self.castling.contains('Q') {
                if castle::Q_VALID & (black_attacks | black_pieces | white_pieces) == 0 {
                    moves.push(Pieces::King(King::from_bits(castle::Q_ZONE, White)));
                }
            }
        } else { // Black
            if white_attacks & king.bits() != 0 {
                check = true;
                moves.clear();
            }
            moves.append(&mut king.moves(&white_attacks, &black_pieces));

            if !check && self.castling.contains('k') {
                if castle::k_VALID & (white_attacks | white_pieces | black_pieces) == 0 {
                    moves.push(Pieces::King(King::from_bits(castle::k_ZONE, Black)));
                }
            }
            if !check && self.castling.contains('q') {
                if castle::q_VALID & (white_attacks | white_pieces | black_pieces) == 0 {
                    moves.push(Pieces::King(King::from_bits(castle::q_ZONE, Black)));
                }
            }
        }

        let opp_diagonal = opp_bishop | opp_queen;
        let opp_straight = opp_rook | opp_queen;
        let mut moves = moves.into_iter().filter(|m| {
            if let Pieces::King(_) = m { return true; }
            filter_pins(king.bits(), &m.bits(), &(opp_diagonal & !m.bits()), &(opp_straight & !m.bits()))
        }).collect::<Vec<Pieces>>();

        if !check { return moves; }

        // determine what pieces are putting king under check
        let mut check_attackers: Vec::<&Pieces> = Vec::new();
        let mut check_attack: u128 = 0x0;
        for piece in &self.pieces {
            let mut attack = 0x0;
            if self.turn == White && *piece.color() == Black {
                for m in piece.moves(&white_pieces, &black_pieces) {
                    attack |= m.bits();
                }
                if attack & king.bits() != 0 {
                    check_attackers.push(piece);
                    check_attack = attack;
                }
            } else if self.turn == Black && *piece.color() == White {
                for m in piece.moves(&black_pieces, &white_pieces) {
                    attack |= m.bits();
                }
                if attack & king.bits() != 0 {
                    check_attackers.push(piece);
                    check_attack = attack;
                }
            }
        }

        /* Cast the king to the attacking peice and see 
         * what the line of attack is to counter it
         */
        let mut check_rays: u128;
        let calc_check_rays = |piece_map: &Pieces| {
            let mut king_rays: u128 = 0x0;
            if self.turn == White {
                for mv in &piece_map.moves(&black_pieces, &white_pieces) {
                    king_rays |= mv.bits();
                }
            } else { // Black
                for mv in &piece_map.moves(&white_pieces, &black_pieces) {
                    king_rays |= mv.bits();
                }
            }
            return king_rays & check_attack;
        };

        // if double check return only king moves
        if check_attackers.len() > 1 { return moves; }

        match check_attackers[0] {
            Pieces::Pawn(_) => {
                let king_map = Pieces::Bishop(Bishop::from_bits(*king.bits(), self.turn));
                check_rays = calc_check_rays(&king_map);
            },
            Pieces::Bishop(_) => {
                let king_map = Pieces::Bishop(Bishop::from_bits(*king.bits(), self.turn));
                check_rays = calc_check_rays(&king_map);
            },
            Pieces::Knight(_) => {
                let king_map = Pieces::Knight(Knight::from_bits(*king.bits(), self.turn));
                check_rays = calc_check_rays(&king_map);
            },
            Pieces::Rook(_) => {
                let king_map = Pieces::Rook(Rook::from_bits(*king.bits(), self.turn));
                check_rays = calc_check_rays(&king_map);
            },
            Pieces::Queen(_) => {
                let king_map = Pieces::Bishop(Bishop::from_bits(*king.bits(), self.turn));
                check_rays = calc_check_rays(&king_map);

                // check if queen is attacking on diagonal, if not use rook attacks
                let queen = check_attackers[0];
                if check_rays & (king.bits() | queen.bits()) != king.bits() | queen.bits() {
                    let king_map = Pieces::Rook(Rook::from_bits(*king.bits(), self.turn));
                    check_rays = calc_check_rays(&king_map);
                }
            },
            _ => panic!("King should not be checking another king"),
        }
        for piece in &self.pieces {
            if let Pieces::King(_) = piece { continue; }

            if self.turn == White && *piece.color() == White {
                for mv in piece.moves(&black_pieces, &white_pieces) {
                    if mv.bits() & check_rays != 0 {
                        moves.push(mv);
                    }
                }
            } else if self.turn == Black && *piece.color() == Black {
                for mv in piece.moves(&white_pieces, &black_pieces) {
                    if mv.bits() & check_rays != 0 {
                        moves.push(mv);
                    }
                }
            }
        }

        /* TODO */
        // fix check rays bug

        return moves;
    }

    pub fn init_boards(&self, white_pieces: &mut u128, black_pieces: &mut u128) {
        for piece in &self.pieces {
            match piece.color() {
                White => *white_pieces |= piece.bits(),
                Black => *black_pieces |= piece.bits(),
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
        let mut ep_remove: usize = 64;
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
        if !ep_this_turn { self.en_passant_square = 0x0; }
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

