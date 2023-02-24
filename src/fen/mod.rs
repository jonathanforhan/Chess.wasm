mod validate;
pub use validate::validate;
mod encode;
//pub use encode::encode;
mod decode;
pub use decode::decode;

use core::fmt;
use std::error::Error;
#[derive(Debug)]
pub struct FenError<'a> {
    pub error: &'a str,
}

impl<'a> fmt::Display for FenError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid Fen")
    }
}

impl<'a> Error for FenError<'a> {
    fn description(&self) -> &'a str {
        &self.error
    }
}
