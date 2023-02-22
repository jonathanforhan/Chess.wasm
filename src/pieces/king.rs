pub struct King {
    bit_board: u128,
}

use super::Piece;
impl Piece for King {
    type T = King;

    fn new(x: usize, y: usize) -> Self::T {
        King { bit_board: 1 << (y << 4) + 8 + x }
    }

    fn moves(&self, opp: &u128, team: &u128) -> Vec<King> {
        /* TODO */
        let p = King::new(0, 0);
        let mut v = Vec::<King>::new();
        v.push(p);
        v
    }
}

