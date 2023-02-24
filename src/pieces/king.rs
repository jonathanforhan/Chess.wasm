use super::{Piece, Color};
use crate::MASK;

pub struct King {
    bits: u128,
    color: Color,
}

impl Piece for King {
    type T = King;

    fn new(x: usize, y: usize, color: Color) -> Self::T {
        King { bits: 1 << (y << 4) + 8 + x, color }
    }

    fn bits(&self) -> &u128 {
        &self.bits
    }

    fn color(&self) -> &Color {
        &self.color
    }

    fn moves(&self, opp: &u128, no_go: &u128) -> Vec<King> {
        /* NOTE
         * nogo MUST be the sum of all oppositision attacks
         * and teammates
         */
        let mut valid_moves = Vec::<King>::new();
        let bits = &self.bits;
        let color = &self.color;

        let mut validate = |test: &u128| {
            if test & MASK == 0 { return; }
            if test & (no_go | opp) != 0 { return; }
            valid_moves.push(King { bits: *test, color: *color });
        };

        let mut test_move = |test: u128| { validate(&test); };

        /* North */
        test_move(bits << 0x10);

        /* South */
        test_move(bits >> 0x10);

        /* West */
        test_move(bits << 0x01);

        /* East */
        test_move(bits >> 0x01);

        /* Northwest */
        test_move(bits << 0x11);

        /* Southeast */
        test_move(bits >> 0x11);

        /* Northeast */
        test_move(bits << 0x0f);

        /* Southwest */
        test_move(bits >> 0x0f);

        return valid_moves;
    }
}

