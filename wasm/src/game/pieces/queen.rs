use super::{Piece, Pieces, Color};
use crate::MASK;

pub struct Queen {
    bits: u128,
    color: Color,
}

impl Queen {
    pub fn new(x: usize, y: usize, color: Color) -> Self {
        Queen { bits: 1 << (y << 4) + 8 + x, color }
    }

    pub fn from_bits(bits: u128, color: Color) -> Self {
        Queen { bits, color }
    }
}

impl Piece for Queen {
    fn bits(&self) -> &u128 {
        &self.bits
    }

    fn color(&self) -> &Color {
        &self.color
    }

    fn set_bits(&mut self, bits: &u128) {
        self.bits = *bits;
    }

    fn moves(&self, opp: &u128, team: &u128) -> Vec<Pieces> {
        let mut valid_moves = Vec::<Pieces>::new();
        let bits = &self.bits;
        let color = &self.color;

        let mut validate = |test: &u128| -> bool {
            if test & MASK == 0 { return false; }
            if test & team != 0 { return false; }
            if test & opp  != 0 { valid_moves.push(Pieces::Queen(Queen { bits: *test | bits, color: *color })); return false; }
            valid_moves.push(Pieces::Queen(Queen { bits: *test | bits, color: *color }));
            return true;
        };

        fn test_move<F, G>(condition: F, validation: &mut G)
            where F: Fn(i32) -> u128, G: FnMut(&u128) -> bool {

            for i in 1..8 {
                let test = condition(i);
                if !validation(&test) { break; }
            }
        }

        /* North */
        test_move(|i: i32| { bits << (i << 4) }, &mut validate);

        /* South */
        test_move(|i: i32| { bits >> (i << 4) }, &mut validate);

        /* West */
        test_move(|i: i32| { bits << i }, &mut validate);

        /* East */
        test_move(|i: i32| { bits >> i }, &mut validate);

        /* Northwest */
        test_move(|i: i32| { bits << (i << 4) + i }, &mut validate);

        /* Southeast */
        test_move(|i: i32| { bits >> (i << 4) + i }, &mut validate);

        /* Northeast */
        test_move(|i: i32| { bits << (i << 4) - i }, &mut validate);

        /* Southwest */
        test_move(|i: i32| { bits >> (i << 4) - i }, &mut validate);

        return valid_moves;
    }
}
