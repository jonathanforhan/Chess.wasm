use crate::{game::{
    pieces::{
        Color::*,
        Piece,
        Pieces,
        Bishop,
        Knight,
        Rook,
        Queen,
    },
    Game,
}};
use super::GameInfo;

// removes moves that are moving a pinned peice
// returns bool to be used in filter method
pub fn filter_pins(info: &GameInfo, game: &Game, mv: &u128) -> bool {
    let diagonal = Pieces::Bishop(Bishop::from_bits(*info.king.bits(), game.turn));
    let straight = Pieces::Rook(Rook::from_bits(*info.king.bits(), game.turn));

    let mut en_passant = 0u128;
    if mv & game.en_passant_square != 0 {
        match game.turn {
            White => en_passant = game.en_passant_square >> 0x10,
            Black => en_passant = game.en_passant_square << 0x10,
        }
    }

    let test_diagonal = info.opp_diagonal & !mv;
    let test_straight = info.opp_straight & !mv;

    for m in diagonal.moves(&(info.opp_pieces & !en_passant), &(info.team_pieces ^ mv)) {
        if m.bits() & test_diagonal != 0 { return false; }
    }
    for m in straight.moves(&(info.opp_pieces & !en_passant), &(info.team_pieces ^ mv)) {
        if m.bits() & test_straight != 0 { return false; }
    }

    return true;
}

pub fn gen_check_moves(info: &GameInfo, game: &Game, moves: &mut Vec<Pieces>) {
    let mut king_rays = 0u128;
    let king_ray_maker = Pieces::Queen(Queen::from_bits(*info.king.bits(), game.turn));
    for mv in king_ray_maker.moves(&info.opp_pieces, &info.team_pieces) {
        king_rays |= mv.bits();
    }
    let king_ray_maker = Pieces::Knight(Knight::from_bits(*info.king.bits(), game.turn));
    for mv in king_ray_maker.moves(&info.opp_pieces, &info.team_pieces) {
        king_rays |= mv.bits();
    }

    let mut attackers: Vec::<&Pieces> = Vec::new();
    let mut attack = 0u128;
    for piece in &game.pieces {
        let mut bits = 0u128;
        if game.turn != *piece.color() && piece.bits() & king_rays != 0 {
            for m in piece.moves(&info.team_pieces, &info.opp_pieces) {
                bits |= m.bits();
            }
            if bits & info.king.bits() != 0 {
                attackers.push(piece);
                attack = bits;
            }
        }
    }

    if attackers.len() > 1 { return; }

    let mut rays;
    /* Gen moves for king as if another peice and & them to
     * the attackers moves to determine the line of attack
     * allows for pieces to block sight of attacker
     */
    let calc_rays = |piece_map: &Pieces, attack: &u128| {
        let mut piece_rays = 0u128;
        for mv in &piece_map.moves(&info.opp_pieces, &info.team_pieces) {
            piece_rays |= mv.bits();
        }
        return piece_rays & attack;
    };

    match attackers[0] {
        Pieces::Pawn(_) |
        Pieces::Bishop(_) => {
            let king_map = Pieces::Bishop(Bishop::from_bits(*info.king.bits(), game.turn));
            rays = calc_rays(&king_map, &attack);
        },
        Pieces::Knight(_) => {
            let king_map = Pieces::Knight(Knight::from_bits(*info.king.bits(), game.turn));
            rays = calc_rays(&king_map, &attack);
        },
        Pieces::Rook(_) => {
            let king_map = Pieces::Rook(Rook::from_bits(*info.king.bits(), game.turn));
            rays = calc_rays(&king_map, &attack);
        },
        Pieces::Queen(_) => {
            attack = 0u128;
            let piece = Pieces::Bishop(Bishop::from_bits(*attackers[0].bits(), game.turn));
            for m in piece.moves(&info.team_pieces, &info.opp_pieces) {
                attack |= m.bits();
            }
            let king_map = Pieces::Bishop(Bishop::from_bits(*info.king.bits(), game.turn));
            rays = calc_rays(&king_map, &attack);

            // check if queen is attacking on diagonal, if not use rook attacks
            let queen = attackers[0];
            if rays & (info.king.bits() | queen.bits()) != info.king.bits() | queen.bits() {
                attack = 0u128;
                let piece = Pieces::Rook(Rook::from_bits(*attackers[0].bits(), game.turn));
                for m in piece.moves(&info.team_pieces, &info.opp_pieces) {
                    attack |= m.bits();
                }
                let king_map = Pieces::Rook(Rook::from_bits(*info.king.bits(), game.turn));
                rays = calc_rays(&king_map, &attack);
            }
        },
        _ => panic!("King should not be checking another king"),
    }


    for piece in &game.pieces {
        if game.turn == *piece.color() {
            if let Pieces::King(_) = piece { continue; }
            for mv in piece.moves(&info.opp_pieces, &info.team_pieces) {
                if mv.bits() & rays != 0 { moves.push(mv); }
            }
        }
    }
}
