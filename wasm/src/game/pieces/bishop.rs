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

    #[inline]
    fn test_move<F, G, T>(&self, condition: F, validation: G, moves: &mut T)
        where F: Fn(i32) -> u128, G: Fn(&u128, &mut T) -> bool {

            for i in 1..8 {
                let test = condition(i);
                if !validation(&test, moves) { break; }
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
        let color = self.color();

        let validate = |test: &u128, moves: &mut Vec<Pieces>| -> bool {
            if test & MASK & !team == 0 { return false; }
            if test & opp != 0 { moves.push(Pieces::Bishop(Bishop { bits: *test | bits, color: *color })); return false; }
            moves.push(Pieces::Bishop(Bishop { bits: *test | bits, color: *color }));
            return true;
        };

        /* Northwest */
        self.test_move(|i: i32| { bits << (i << 4) + i }, validate, moves);

        /* Southeast */
        self.test_move(|i: i32| { bits >> (i << 4) + i }, validate, moves);

        /* Northeast */
        self.test_move(|i: i32| { bits << (i << 4) - i }, validate, moves);

        /* Southwest */
        self.test_move(|i: i32| { bits >> (i << 4) - i }, validate, moves);
    }

    fn moves_as_bits(&self, opp: &u128, team: &u128, moves: &mut u128) {
        let bits = &self.bits;

        let validate = |test: &u128, moves: &mut u128| -> bool {
            if test & MASK & !team == 0 { return false; }
            if test & opp != 0 { *moves |= *test | bits; return false; }
            *moves |= *test | bits;
            return true;
        };

        /* Northwest */
        self.test_move(|i: i32| { bits << (i << 4) + i }, validate, moves);

        /* Southeast */
        self.test_move(|i: i32| { bits >> (i << 4) + i }, validate, moves);

        /* Northeast */
        self.test_move(|i: i32| { bits << (i << 4) - i }, validate, moves);

        /* Southwest */
        self.test_move(|i: i32| { bits >> (i << 4) - i }, validate, moves);
    }

    fn moves_as_bits_exclusive(&self, opp: &u128, team: &u128, moves: &mut u128) {
        let bits = &self.bits;

        let validate = |test: &u128, moves: &mut u128| -> bool {
            if test & MASK & !team == 0 { return false; }
            if test & opp != 0 { *moves |= *test; return false; }
            *moves |= *test;
            return true;
        };

        /* Northwest */
        self.test_move(|i: i32| { bits << (i << 4) + i }, validate, moves);

        /* Southeast */
        self.test_move(|i: i32| { bits >> (i << 4) + i }, validate, moves);

        /* Northeast */
        self.test_move(|i: i32| { bits << (i << 4) - i }, validate, moves);

        /* Southwest */
        self.test_move(|i: i32| { bits >> (i << 4) - i }, validate, moves);
    }
}
