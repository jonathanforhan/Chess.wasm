use super::{Piece, Pieces, Color};
use crate::MASK;

#[derive(Clone)]
pub struct King {
    bits: u128,
    color: Color,
}

impl King {
    pub fn new(x: usize, y: usize, color: Color) -> Self {
        King { bits: 1 << (y << 4) + 8 + x, color }
    }

    // from-bits exists for faster creation like castling and promotions
    // this is to speed up move gen
    pub fn from_bits(bits: u128, color: Color) -> Self {
        King { bits, color }
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

    fn moves(&self, opp: &u128, team: &u128, moves: &mut Vec<Pieces>) {
        /* NOTE
         * opp MUST be the sum of all oppositision attacks
         * and teammates
         */
        let bits = &self.bits;
        let color = &self.color;

        let mut validate = |test: &u128| {
            if test & MASK == 0 { return; }
            if test & (team | opp) != 0 { return; }
            moves.push(Pieces::King(King { bits: *test | bits, color: *color }));
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
    }

    fn moves_as_bits(&self, opp: &u128, team: &u128, moves: &mut u128) {
        /* NOTE
         * opp MUST be the sum of all oppositision attacks
         * and teammates
         */
        let bits = &self.bits;

        let mut validate = |test: &u128| {
            if test & MASK == 0 { return; }
            if test & (team | opp) != 0 { return; }
            *moves |= *test | bits;
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
    }

    fn moves_as_bits_exclusive(&self, opp: &u128, team: &u128, moves: &mut u128) {
        /* NOTE
         * opp MUST be the sum of all oppositision attacks
         * and teammates
         */
        let bits = &self.bits;

        let mut validate = |test: &u128| {
            if test & MASK == 0 { return; }
            if test & (team | opp) != 0 { return; }
            *moves |= *test;
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
    }
}
