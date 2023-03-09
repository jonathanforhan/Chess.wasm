use crate::game::{
    pieces::{
        Color,
        Piece,
        Pieces,
        Bishop,
        Knight,
        Rook,
    },
    Game,
};
use super::GameInfo;

// removes moves that are moving a pinned peice
// returns bool to be used in filter method
pub fn filter_pins(info: &GameInfo, turn: Color, mv: &u128) -> bool {
    let diagonal = Pieces::Bishop(Bishop::from_bits(*info.king.bits(), turn));
    let straight = Pieces::Rook(Rook::from_bits(*info.king.bits(), turn));
    let test_diagonal = info.opp_diagonal & !mv;
    let test_straight = info.opp_straight & !mv;

    for m in diagonal.moves(&test_diagonal, &(info.team_pieces ^ mv)) {
        if m.bits() & test_diagonal != 0 { return false; }
    }
    for m in straight.moves(&test_straight, &(info.team_pieces ^ mv)) {
        if m.bits() & test_straight != 0 { return false; }
    }
    return true;
}

pub fn gen_check_moves(info: &GameInfo, game: &Game) -> Vec<Pieces> {
    let mut attackers: Vec::<&Pieces> = Vec::new();
    let mut attack = 0u128;
    for piece in &game.pieces {
        let mut bits = 0u128;
        if game.turn != *piece.color() {
            for m in piece.moves(&info.team_pieces, &info.opp_pieces) {
                bits |= m.bits();
            }
            if bits & info.king.bits() != 0 {
                attackers.push(piece);
                attack = bits;
            }
        }
    }

    if attackers.len() > 1 { return vec![]; }

    let mut rays;
    /* Gen moves for king as if another peice and & them to
     * the attackers moves to determine the line of attack
     * allows for pieces to block sight of attacker
     */
    let calc_rays = |piece_map: &Pieces| {
        let mut king_rays = 0u128;
        for mv in &piece_map.moves(&info.opp_pieces, &info.team_pieces) {
            king_rays |= mv.bits();
        }
        return king_rays & attack;
    };

    match attackers[0] {
        Pieces::Pawn(_) |
        Pieces::Bishop(_) => {
            let king_map = Pieces::Bishop(Bishop::from_bits(*info.king.bits(), game.turn));
            rays = calc_rays(&king_map);
        },
        Pieces::Knight(_) => {
            let king_map = Pieces::Knight(Knight::from_bits(*info.king.bits(), game.turn));
            rays = calc_rays(&king_map);
        },
        Pieces::Rook(_) => {
            let king_map = Pieces::Rook(Rook::from_bits(*info.king.bits(), game.turn));
            rays = calc_rays(&king_map);
        },
        Pieces::Queen(_) => {
            let king_map = Pieces::Bishop(Bishop::from_bits(*info.king.bits(), game.turn));
            rays = calc_rays(&king_map);

            // check if queen is attacking on diagonal, if not use rook attacks
            let queen = attackers[0];
            if rays & (info.king.bits() | queen.bits()) != info.king.bits() | queen.bits() {
                let king_map = Pieces::Rook(Rook::from_bits(*info.king.bits(), game.turn));
                rays = calc_rays(&king_map);
            }
        },
        _ => panic!("King should not be checking another king"),
    }

    let mut moves = Vec::new();
    for piece in &game.pieces {
        if game.turn == *piece.color() {
            if let Pieces::King(_) = piece { continue; }
            for mv in piece.moves(&info.opp_pieces, &info.team_pieces) {
                if mv.bits() & rays != 0 { moves.push(mv); }
            }
        }
    }

    return moves;
}
