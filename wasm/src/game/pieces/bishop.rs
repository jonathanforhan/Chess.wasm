use super::{Piece, Pieces, Color};
use crate::game::MASK;

#[derive(Clone)]
pub struct Bishop {
    bits: u128,
    color: Color,
}

impl Bishop {
    pub fn new(x: usize, y: usize, color: Color) -> Self {
        Bishop { bits: 1 << (y << 4) + 8 + x, color }
    }

    pub fn from_bits(bits: u128, color: Color) -> Self {
        Bishop { bits, color }
    }

    fn test_move<F, G>(&self, condition: F, validation: &mut G)
        where F: Fn(i32) -> u128, G: FnMut(&u128) -> bool {

            for i in 1..8 {
                let test = condition(i);
                if !validation(&test) { break; }
            }
        }
}

impl Piece for Bishop {
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
        let bits = &self.bits;

        let mut validate = |test: &u128| -> bool {
            if test & MASK == 0 { return false; }
            if test & team != 0 { return false; }
            if test & opp  != 0 { moves.push(Pieces::Bishop(Bishop { bits: *test | bits, color: self.color })); return false; }
            moves.push(Pieces::Bishop(Bishop { bits: *test | bits, color: self.color }));
            return true;
        };

        /* Northwest */
        self.test_move(|i: i32| { bits << (i << 4) + i }, &mut validate);

        /* Southeast */
        self.test_move(|i: i32| { bits >> (i << 4) + i }, &mut validate);

        /* Northeast */
        self.test_move(|i: i32| { bits << (i << 4) - i }, &mut validate);

        /* Southwest */
        self.test_move(|i: i32| { bits >> (i << 4) - i }, &mut validate);
    }

    fn moves_as_bits(&self, opp: &u128, team: &u128, moves: &mut u128) {
        let bits = &self.bits;

        let mut validate = |test: &u128| -> bool {
            if test & MASK == 0 { return false; }
            if test & team != 0 { return false; }
            if test & opp  != 0 { *moves |= *test | bits; return false; }
            *moves |= *test | bits;
            return true;
        };

        /* Northwest */
        self.test_move(|i: i32| { bits << (i << 4) + i }, &mut validate);

        /* Southeast */
        self.test_move(|i: i32| { bits >> (i << 4) + i }, &mut validate);

        /* Northeast */
        self.test_move(|i: i32| { bits << (i << 4) - i }, &mut validate);

        /* Southwest */
        self.test_move(|i: i32| { bits >> (i << 4) - i }, &mut validate);
    }

    fn moves_as_bits_exclusive(&self, opp: &u128, team: &u128, moves: &mut u128) {
        let bits = &self.bits;

        let mut validate = |test: &u128| -> bool {
            if test & MASK == 0 { return false; }
            if test & team != 0 { return false; }
            if test & opp  != 0 { *moves |= *test; return false; }
            *moves |= *test;
            return true;
        };

        /* Northwest */
        self.test_move(|i: i32| { bits << (i << 4) + i }, &mut validate);

        /* Southeast */
        self.test_move(|i: i32| { bits >> (i << 4) + i }, &mut validate);

        /* Northeast */
        self.test_move(|i: i32| { bits << (i << 4) - i }, &mut validate);

        /* Southwest */
        self.test_move(|i: i32| { bits >> (i << 4) - i }, &mut validate);
    }
}
