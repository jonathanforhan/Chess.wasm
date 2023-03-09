use crate::game::{
    Game,
    util::{
        GameInfo,
        promote
    },
    pieces::{
        Piece,
        Pieces,
    },
};

pub fn gen_moves(info: &mut GameInfo, game: &Game) -> Vec<Pieces> {
    let mut moves: Vec::<Pieces> = Vec::new();
    for piece in &game.pieces {
        if game.turn == *piece.color() {
            match piece {
                Pieces::King(_) => (),
                Pieces::Pawn(_) => {
                    let mut mvs = piece.moves(&(info.opp_pieces | game.en_passant_square), &info.team_pieces);
                    // adds promotion options to mvs
                    mvs.append(&mut promote::add_promotions(&mvs, game.turn));
                    moves.append(&mut mvs);
                },
                _ => moves.append(&mut piece.moves(&info.opp_pieces, &info.team_pieces)),
            }
        } else { // Opp
            if let Pieces::Pawn(p) = piece {
                for a in p.attacks() { info.opp_attacks |= a.bits(); }
                continue;
            }
            // enemy king should never come near king so all 
            // king radius squares are opp attack squares
            if let Pieces::King(k) = piece {
                for a in k.moves(&0u128, &0u128) { info.opp_attacks |= a.bits(); }
            }

            for m in piece.moves(&((info.team_pieces & !info.king.bits()) | info.opp_pieces), &0u128) {
                info.opp_attacks |= m.bits() ^ piece.bits();
                match m {
                    Pieces::Bishop(_) =>
                        info.opp_diagonal |= piece.bits(),
                    Pieces::Rook(_) =>
                        info.opp_straight |= piece.bits(),
                    Pieces::Queen(_) => {
                        info.opp_diagonal |= piece.bits();
                        info.opp_straight |= piece.bits();
                    }
                    _ => ()
                }
            }
        }
    }
    moves
}

