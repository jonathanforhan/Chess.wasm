pub mod constants {
    // the en passant src and dst rows for corresponding colors
    pub const WHITE_EN_PASSANT: u128 = 0xff00 << 0x10 | 0xff00 << 0x30;
    pub const BLACK_EN_PASSANT: u128 = 0xff00 << 0x60 | 0xff00 << 0x40;
}
pub use constants::*;

pub fn add_en_passant (mv: &u128) -> Option<u128> {
    if mv & WHITE_EN_PASSANT == *mv {
        let ep = mv & (0xff00 << 0x10);
        return Some(ep << 0x10);
    }
    else if mv & BLACK_EN_PASSANT == *mv {
        let ep = mv & (0xff00 << 0x60);
        return Some(ep >> 0x10);
    }
    return None;
}
