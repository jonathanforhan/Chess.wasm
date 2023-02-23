use super::{Piece, Color};

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

    fn moves(&self, opp: &u128, team: &u128) -> Vec<King> {
        /* NOTE
         * opp MUST be the sum of all oppositision attacks
         * NOT a piece map
         */
        let mut valid_moves = Vec::<King>::new();
        let bits = &self.bits;
        let color = &self.color;

        let mut validate = |test: &u128| {
            if test & (team | opp) != 0 { return; }
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

