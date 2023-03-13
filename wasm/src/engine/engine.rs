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
        let moves = game.moves();
        if moves.is_empty() {
            return Err(Box::new(EngineError("No moves".into())));
        }

        let result = Arc::new(Mutex::new(Vec::<(usize, i32)>::new()));

        moves.par_iter().enumerate().for_each(|(i, mv)| {
            let mut game_copy = game.clone();
            game_copy.move_piece(*mv.bits());
            let eval = minimax(game_copy, false, 4, i32::MIN, i32::MAX);
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
