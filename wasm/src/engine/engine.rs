use super::{EngineError, minimax};
use std::{error::Error, sync::{Arc, Mutex}};
use crate::game::{
    pieces::{
        Piece,
        Pieces,
    },
    fen
};
use rayon::prelude::*;

pub struct Engine();

impl Engine {
    pub fn best_move(fen: String) -> Result<Pieces, Box<dyn Error>> {
        let game = fen::decode(&fen)?;
        let (moves, info) = game.moves_verbose()?;
        if info.checkmate {
            return Err(Box::new(EngineError("Checkmate".into())));
        }
        if info.stalemate {
            return Err(Box::new(EngineError("Draw".into())));
        }

        let result = Arc::new(Mutex::new(Vec::<(usize, i32)>::new()));

        // less moves to calc means
        // greater depth possible
        let mut cost = 0u8; // if the calc is expensive
        for piece in &game.pieces {
            match piece {
                Pieces::Pawn(_) => cost += 1,
                Pieces::Bishop(_) => cost += 3,
                Pieces::Knight(_) => cost += 2,
                Pieces::Rook(_) => cost += 3,
                Pieces::Queen(_) => cost += 7,
                _ => (),
            }
        }

        let depth;
        if cost > 50 {
            depth = 1;
        } else if cost > 34 {
            depth = 2;
        } else if cost > 18 {
            depth = 3;
        } else {
            depth = 4;
        }

        // the leaves will be different at odd
        // and even depths so a factor is applied
        let mut factor = 1;
        if depth % 2 == 0 {
            factor = -1;
        }

        moves.par_iter().enumerate().for_each(|(i, mv)| {
            let mut game_copy = game.clone();
            game_copy.move_piece(*mv.bits());
            let eval = minimax(game_copy.clone(), false, depth, i32::MIN, i32::MAX, factor).unwrap_or_else(|_| {
                minimax(game_copy.clone(), false, depth-1, i32::MIN, i32::MAX, -factor).unwrap()
            });
            let result_copy = result.clone();
            result_copy.lock().unwrap().push((i, eval));
        });

        let mut best_move = (0, i32::MIN);
        for mv in result.lock().unwrap().iter() {
            if mv.1 > best_move.1 {
                best_move = (mv.0, mv.1);
            }
        }

        Ok(moves[best_move.0].clone())
    }
}
