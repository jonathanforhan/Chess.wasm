use super::{Piece, Pieces, Color};
use crate::game::MASK;

#[derive(Clone)]
pub struct Knight {
    bits: u128,
    color: Color
}

impl Knight {
    pub fn new(x: usize, y: usize, color: Color) -> Self {
        Knight { bits: 1 << (y << 4) + 8 + x, color }
    }

    pub fn from_bits(bits: u128, color: Color) -> Self {
        Knight { bits, color }
    }
}

impl Piece for Knight {
    fn bits(&self) -> &u128 {
        &self.bits
    }

    fn color(&self) -> &Color {
        &self.color
    }

    fn set_bits(&mut self, bits: &u128) {
        self.bits = *bits;
    }

    fn moves(&self, _: &u128, team: &u128, moves: &mut Vec<Pieces>) {
        let bits = &self.bits;

        let mut validate = |test: u128| {
            if test & MASK == 0 { return; }
            if test & team != 0 { return; }
            moves.push(Pieces::Knight(Knight { bits: test | bits, color: self.color }));
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
    }

    fn moves_as_bits(&self, _: &u128, team: &u128, moves: &mut u128) {
        let bits = &self.bits;

        let mut validate = |test: u128| {
            if test & MASK == 0 { return; }
            if test & team != 0 { return; }
            *moves |= test | bits;
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
    }

    fn moves_as_bits_exclusive(&self, _: &u128, team: &u128, moves: &mut u128) {
        let bits = &self.bits;

        let mut validate = |test: u128| {
            if test & MASK == 0 { return; }
            if test & team != 0 { return; }
            *moves |= test;
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
    }
}
