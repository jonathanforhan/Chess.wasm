use std::fmt;
use std::error::Error;

mod validate;
pub use validate::validate;
mod encode;
pub use encode::encode;
mod decode;
pub use decode::decode;

#[derive(Debug)]
pub struct FenError(String);

impl fmt::Display for FenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fen Error: {}", self.0)
    }
}

impl Error for FenError {}
