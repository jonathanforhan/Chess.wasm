use crate::game::{Game, pieces::{Piece, Pieces}, util::GameInfo};

#[must_use]
pub fn evaluate(game: &Game, info: &GameInfo) -> i32 {
    let mut eval = 0i32;

    if info.check && info.valid_moves == 0 {
        eval -= 1000;
    }

    if info.check {
        eval -= 2;
    }


    for piece in &game.pieces {
        if *piece.color() == game.turn {
            // early game
            if game.move_count < 8 {
                if *piece.bits() & 0xffff_ffff_7e00_7e00_7e00_7e00_0000_0000 != 0 {
                    eval += 1;
                }
            }
            match piece {
                Pieces::Pawn(_) => eval += 2,
                Pieces::Bishop(_) => eval += 6,
                Pieces::Knight(_) => eval += 6,
                Pieces::Rook(_) => eval += 10,
                Pieces::Queen(_) => eval += 18,
                Pieces::King(_) => (),
            }
        } else {
            match piece {
                Pieces::Pawn(_) => eval -= 2,
                Pieces::Bishop(_) => eval -= 6,
                Pieces::Knight(_) => eval -= 6,
                Pieces::Rook(_) => eval -= 10,
                Pieces::Queen(_) => eval -= 18,
                Pieces::King(_) => (),
            }
        }
    }

    return eval;
}
