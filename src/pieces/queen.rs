pub struct Queen {
    bit_board: u128,
}

use super::Piece;
impl Piece for Queen {
    type T = Queen;

    fn new(x: usize, y: usize) -> Self::T {
        Queen { bit_board: 1 << (y << 4) + 8 + x }
    }

    fn moves(&self, opp: &u128, team: &u128) -> Vec<Queen> {
        /* TODO */
        let p = Queen::new(0, 0);
        let mut v = Vec::<Queen>::new();
        v.push(p);
        v
    }
}

