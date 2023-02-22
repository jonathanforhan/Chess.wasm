pub struct Pawn {
    bit_board: u128,
}

use super::Piece;
impl Piece for Pawn {
    type T = Pawn;

    fn new(x: usize, y: usize) -> Self::T {
        Pawn { bit_board: 1 << (y << 4) + 8 + x }
    }

    fn moves(&self, opp: &u128, team: &u128) -> Vec<Pawn> {
        /* TODO */
        let p = Pawn::new(0, 0);
        let mut v = Vec::<Pawn>::new();
        v.push(p);
        v
    }
}
