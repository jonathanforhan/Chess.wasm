pub struct Bishop {
    bits: u128,
}

use super::Piece;
impl Piece for Bishop {
    type T = Bishop;

    fn new(x: usize, y: usize) -> Self::T {
        Bishop { bits: 1 << (y << 4) + 8 + x }
    }

    fn moves(&self, opp: &u128, team: &u128) -> Vec<Bishop> {
        let mut v = Vec::<Bishop>::new();

        let mut test_move = |t: &u128| -> bool {
            if t & team != 0 { return false; }
            if t & opp  != 0 { v.push(Bishop { bits: *t }); return false; }
            v.push(Bishop { bits: *t });
            return true;
        };

        /* Northwest */
        for i in 1..8 {
            let test = &self.bits << (i << 4) + i;
            if !test_move(&test) { break; }
        }
        /* Southeast */
        for i in 1..8 {
            let test = &self.bits >> (i << 4) + i;
            if !test_move(&test) { break; }
        }
        /* Northeast */
        for i in 1..8 {
            let test = &self.bits << (i << 4) - i;
            if !test_move(&test) { break; }
        }
        /* Southwest */
        for i in 1..8 {
            let test = &self.bits >> (i << 4) - i;
            if !test_move(&test) { break; }
        }

        return v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_bits() {
        let b = Bishop::new(4, 4);
        let opp = 0u128;
        let team = 0u128;
        let v = b.moves(&opp, &team);

        let mut x: u128 = 0;

        for n in v.iter() {
            x ^= n.bits;
        }

        for i in (-15..=112).rev().step_by(16) { // 0..128 but with rev-step
            for j in 0..8 {
                print!("{} ", (x >> (i + j + 8) & 1)
                       .to_string()
                       .replace('1', &'B'.to_string())
                       .replace('0', "*"));
            }
            println!(". . . . . . . .");
        }
        print!("\n");

    }
}
