pub mod constants {
    // the en passant src and dst rows for corresponding colors
    pub const WHITE_EN_PASSANT: u128 = 0xff00 << 0x10 | 0xff00 << 0x30;
    pub const BLACK_EN_PASSANT: u128 = 0xff00 << 0x60 | 0xff00 << 0x40;
}
pub use constants::*;
