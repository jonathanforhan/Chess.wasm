use crate::game::{
    Game,
    pieces::Piece,
};
use std::cmp;

use super::evaluate;


pub fn minimax(game: Game, maximizer: bool, depth: u8, mut alpha: i32, mut beta: i32) -> i32 {
    let (moves, info) = game.moves_verbose();
    let moves = moves.iter().map(|x| {
        *x.bits()
    }).collect::<Vec<u128>>();

    if depth <= 1 || info.valid_moves == 0 {
        return evaluate(&game, &info);
    }

    if maximizer {
        let mut best = i32::MIN;

        for mv in moves {
            let mut game_copy = game.clone();
            game_copy.move_piece(mv);
            best = cmp::max(best, minimax(game_copy, false, depth-1, alpha, beta));
            alpha = cmp::max(alpha, best);
            if beta <= alpha {
                break;
            }
        }
        return best;
    }

    else {
        let mut best = i32::MAX;

        for mv in moves {
            let mut game_copy = game.clone();
            game_copy.move_piece(mv);
            best = cmp::min(best, minimax(game_copy, true, depth-1, alpha, beta));
            beta = cmp::min(beta, best);
            if beta <= alpha {
                break;
            }
        }
        return best;
    }
}
