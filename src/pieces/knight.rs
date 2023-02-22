pub struct Knight {
    bit_board: u128,
}

use super::Piece;
impl Piece for Knight {
    type T = Knight;

    fn new(x: usize, y: usize) -> Self::T {
        Knight { bit_board: 1 << (y << 4) + 8 + x }
    }

    fn moves(&self, opp: &u128, team: &u128) -> Vec<Knight> {
        /* TODO */
        let p = Knight::new(0, 0);
        let mut v = Vec::<Knight>::new();
        v.push(p);
        v
    }
}

