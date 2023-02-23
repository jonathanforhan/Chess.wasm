use super::{Piece, Color};

pub struct Bishop {
    bits: u128,
    color: Color,
}

impl Piece for Bishop {
    type T = Bishop;

    fn new(x: usize, y: usize, color: Color) -> Self::T {
        Bishop { bits: 1 << (y << 4) + 8 + x, color }
    }

    fn bits(&self) -> &u128 {
        &self.bits
    }

    fn moves(&self, opp: &u128, team: &u128) -> Vec<Bishop> {
        let mut valid_moves = Vec::<Bishop>::new();
        let bits = &self.bits;

        let mut validate = |test: &u128| -> bool {
            if test & team != 0 { return false; }
            if test & opp  != 0 { valid_moves.push(Bishop { bits: *test, color: self.color }); return false; }
            valid_moves.push(Bishop { bits: *test, color: self.color });
            return true;
        };

        fn test_move<F, G>(condition: F, validation: &mut G)
            where F: Fn(i32) -> u128, G: FnMut(&u128) -> bool {

            for i in 1..8 {
                let test = condition(i);
                if !validation(&test) { break; }
            }
        }

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
