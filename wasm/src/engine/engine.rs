use super::EngineError;
use std::error::Error;
use crate::game::{self, pieces::Pieces, fen};

pub struct Engine {
    fen: String
}

impl Engine {
    fn best_move<'a>(fen: String) -> Result<Pieces, Box<dyn Error>> {
        let game = fen::decode(&fen)?;
        let moves = game.moves();

        Err(Box::new(EngineError(format!("TODO"))))
    }
}
