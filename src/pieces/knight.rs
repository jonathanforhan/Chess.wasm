use super::{Piece, Color};
use crate::MASK;

pub struct Knight {
    bits: u128,
    color: Color
}

impl Piece for Knight {
    type T = Knight;

    fn new(x: usize, y: usize, color: Color) -> Self::T {
        Knight { bits: 1 << (y << 4) + 8 + x, color }
    }

    fn bits(&self) -> &u128 {
        &self.bits
    }

    fn color(&self) -> &Color {
        &self.color
    }

    fn moves(&self, _: &u128, team: &u128) -> Vec<Knight> {
        let mut valid_moves = Vec::<Knight>::new();
        let bits = &self.bits;

        let mut validate = |test: u128| {
            if test & MASK == 0 { return; }
            if test & team != 0 { return; }
            valid_moves.push(Knight { bits: test, color: self.color });
        };

        let mut test_move = |x: u128| {
            validate(bits << x);
            validate(bits >> x);
        };

        // Knight magic numbers
        test_move(0x21);
        test_move(0x1f);
        test_move(0x12);
        test_move(0x0e);

        return valid_moves;
    }
}

