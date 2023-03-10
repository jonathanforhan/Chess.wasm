use crate::game::{Game, pieces::{Piece, Pieces}, util::GameInfo};

pub const PAWN_VAL: i32 = 2;
pub const BISHOP_VAL: i32 = 6;
pub const KNIGHT_VAL: i32 = 6;
pub const ROOK_VAL: i32 = 10;
pub const QUEEN_VAL: i32 = 18;
pub const HOT_ZONE: u128 = 0x0000_0000_2400_3c00_3c00_2400_0000_0000;
pub const KING_SAFETY: u128 = 0x42 << 0x08 | 0x42 << 0x78;

#[derive(PartialEq)]
enum GameState {
    Early,
    Mid,
    Late
}

#[must_use]
pub fn evaluate(game: &Game, info: &GameInfo, factor: i32) -> i32 {
    let mut eval = 0i32;

    let game_state = match game.move_count {
        0..=16 => GameState::Early,
        17..=32 => GameState::Mid,
        _ => GameState::Late,
    };

    if info.double_check == true {
        eval -= 50;
    }
    else if info.check == true {
        eval -= 2;
    }
    if info.valid_moves < 3 {
        eval -= 5;
    }
    if info.check == true && info.valid_moves < 3 {
        eval -= 20;
    }

    for piece in &game.pieces {
        if *piece.color() == game.turn {
            match piece {
                Pieces::Pawn(p) => {
                    eval += PAWN_VAL;
                    if game_state == GameState::Early && p.bits() & HOT_ZONE != 0 {
                        eval += 1;
                    }
                },
                Pieces::Bishop(b) => {
                    eval += BISHOP_VAL;
                    if game_state == GameState::Early && b.bits() & HOT_ZONE != 0 {
                        eval += 1;
                    }
                },
                Pieces::Knight(n) => {
                    eval += KNIGHT_VAL;
                    if game_state == GameState::Early && n.bits() & HOT_ZONE != 0 {
                        eval += 1;
                    }
                },
                Pieces::Rook(_) => {
                    eval += ROOK_VAL;
                },
                Pieces::Queen(_) => {
                    eval += QUEEN_VAL;
                },
                Pieces::King(k) => {
                    if k.bits() & KING_SAFETY != 0 {
                        eval += 2;
                    }
                },
            }
        } else { // opp
            match piece {
                Pieces::Pawn(p) => {
                    eval -= PAWN_VAL;
                    if game_state == GameState::Early && p.bits() & HOT_ZONE != 0 {
                        eval -= 1;
                    }
                },
                Pieces::Bishop(b) => {
                    eval -= BISHOP_VAL;
                    if game_state == GameState::Early && b.bits() & HOT_ZONE != 0 {
                        eval -= 1;
                    }
                },
                Pieces::Knight(n) => {
                    eval -= KNIGHT_VAL;
                    if game_state == GameState::Early && n.bits() & HOT_ZONE != 0 {
                        eval -= 1;
                    }
                },
                Pieces::Rook(_) => {
                    eval -= ROOK_VAL;
                },
                Pieces::Queen(_) => {
                    eval -= QUEEN_VAL;
                },
                Pieces::King(k) => {
                    if k.bits() & KING_SAFETY != 0 {
                        eval -= 2;
                    }
                },
            }
        }
    }

    if info.checkmate == true {
        eval = -1_000_000;
    }

    // factor used for variable depths
    return eval * factor;
}
