use std::{
    error::Error,
    fmt
};

pub mod engine;

#[derive(Debug)]
pub struct EngineError(String);

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Engine Error: {}", self.0)
    }
}

impl Error for EngineError {}
