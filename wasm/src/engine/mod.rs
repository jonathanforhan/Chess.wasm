use std::{
    error::Error,
    fmt
};

pub mod engine;
pub use engine::Engine;
mod minimax;
pub use minimax::minimax;
mod evaluate;
pub use evaluate::evaluate;

#[derive(Debug)]
pub struct EngineError(String);

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Engine Error: {}", self.0)
    }
}

impl Error for EngineError {}
