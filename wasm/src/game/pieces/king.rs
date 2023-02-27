use super::{Piece, Pieces, Color};
use crate::MASK;

pub struct King {
    bits: u128,
    color: Color,
}

impl King {
    pub fn new(x: usize, y: usize, color: Color) -> Self {
        King { bits: 1 << (y << 4) + 8 + x, color }
    }
}

impl Piece for King {
    fn bits(&self) -> &u128 {
        &self.bits
    }

    fn color(&self) -> &Color {
        &self.color
    }

    fn set_bits(&mut self, bits: &u128) {
        self.bits = *bits;
    }

    fn moves(&self, opp: &u128, no_go: &u128) -> Vec<Pieces> {
        /* NOTE
         * nogo MUST be the sum of all oppositision attacks
         * and teammates
         */
        let mut valid_moves = Vec::<Pieces>::new();
        let bits = &self.bits;
        let color = &self.color;

        let mut validate = |test: &u128| {
            if test & MASK == 0 { return; }
            if test & (no_go | opp) != 0 { return; }
            valid_moves.push(Pieces::King(King { bits: *test | bits, color: *color }));
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