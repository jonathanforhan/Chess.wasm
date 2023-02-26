pub mod algebraic;
pub use algebraic::{
    bits_to_algebraic,
    algebraic_to_bits,
};

use core::fmt;
use std::error::Error;
#[derive(Debug)]
pub struct NotationError<'a> {
    pub error: &'a str,
}

impl<'a> fmt::Display for NotationError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid Notation")
    }
}

impl<'a> Error for NotationError<'a> {
    fn description(&self) -> &'a str {
        &self.error
    }
}
