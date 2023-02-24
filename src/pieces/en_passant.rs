use super::Color;

pub struct EnPassant {
    bits: u128,
    color: Color,
}

// DOES NOT impl Piece trait
impl EnPassant {
    pub fn new(x: usize, y: usize, color: Color) -> Self {
        EnPassant { bits: 1 << (y << 4) + 8 + x, color }
    }

    pub fn bits(&self) -> &u128 {
        &self.bits
    }

    pub fn color(&self) -> &Color {
        &self.color
    }
}

