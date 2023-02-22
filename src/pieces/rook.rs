pub struct Rook {
    bit_board: u128,
}

use super::Piece;
impl Piece for Rook {
    type T = Rook;

    fn new(x: usize, y: usize) -> Self::T {
        Rook { bit_board: 1 << (y << 4) + 8 + x }
    }

    fn moves(&self, opp: &u128, team: &u128) -> Vec<Rook> {
        /* TODO */
        let p = Rook::new(0, 0);
        let mut v = Vec::<Rook>::new();
        v.push(p);
        v
    }
}

