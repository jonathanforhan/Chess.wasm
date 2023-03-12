use crate::game::{
    Game,
    util::GameInfo,
    pieces::{
        Piece,
        Pieces,
    },
};

pub fn gen_moves(info: &mut GameInfo, game: &Game, moves: &mut Vec<Pieces>) {
    for piece in &game.pieces {
        if game.turn == *piece.color() {
            match piece {
                Pieces::King(_) => (),
                Pieces::Pawn(p) => {
                    let opp = info.opp_pieces | game.en_passant_square;
                    p.moves(&opp, &info.team_pieces, moves);
                },
                _ => piece.moves(&info.opp_pieces, &info.team_pieces, moves),
            }
        } else { // Opp
            match piece {
                Pieces::Pawn(p) => {
                    p.attacks(&mut info.opp_attacks);
                    continue;
                },
                Pieces::King(k) => {
                    // enemy king should never come near king so all 
                    // king radius squares are opp attack squares
                    k.moves_as_bits_exclusive(&0u128, &0u128, &mut info.opp_attacks);
                    continue;
                }
                Pieces::Knight(n) => {
                    let opp = (info.team_pieces & !info.king.bits()) | info.opp_pieces;
                    n.moves_as_bits_exclusive(&opp, &0u128, &mut info.opp_attacks);
                }
                Pieces::Bishop(b) => {
                    let opp = (info.team_pieces & !info.king.bits()) | info.opp_pieces;
                    b.moves_as_bits_exclusive(&opp, &0u128, &mut info.opp_attacks);
                    info.opp_diagonal |= b.bits();
                }
                Pieces::Rook(r) => {
                    let opp = (info.team_pieces & !info.king.bits()) | info.opp_pieces;
                    r.moves_as_bits_exclusive(&opp, &0u128, &mut info.opp_attacks);
                    info.opp_straight |= r.bits();
                }
                Pieces::Queen(q) => {
                    let opp = (info.team_pieces & !info.king.bits()) | info.opp_pieces;
                    q.moves_as_bits_exclusive(&opp, &0u128, &mut info.opp_attacks);
                    info.opp_diagonal |= q.bits();
                    info.opp_straight |= q.bits();
                }
            }
        }
    }
}
