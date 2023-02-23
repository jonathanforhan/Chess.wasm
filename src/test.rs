use super::*;

#[cfg(test)]
mod tests {
    use wasm_bindgen::UnwrapThrowExt;

    use super::*;
    /* cargo test [TEST NAME] -- --nocapture */
    #[test]
    fn test_fen() {
        fen::validate("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").unwrap();
    }

    #[test]
    fn test_moves() {
        let piece = BitBoard::King(King::new(1, 0, Color::White));
        let k;
        if let BitBoard::King(x) = piece { k = x; } else { panic!("BitBoard initialization error"); }
        let piece = BitBoard::Queen(Queen::new(7, 7, Color::White));
        let q;
        if let BitBoard::Queen(x) = piece { q = x; } else { panic!("BitBoard initialization error"); }
        
        let (o, t) = (0, 0);
        let qv = q.moves(&o, &t);
        let mut qm: u128 = 0;
        for n in qv { qm ^= n.bits(); }

        let (o, t) = (qm, 0);
        let kv = k.moves(&o, &t);
        let mut km: u128 = 0;
        for n in kv { km ^= n.bits(); }

        print_bits(&qm, 'q');
        print_bits(&km, 'k');
    }

    fn print_bits(x: &u128, c: char) {
        for i in (-15..=112).rev().step_by(16) { // 0..128 but with rev-step
            for j in 0..8 {
                print!("{} ", (x >> (i + j + 8) & 1)
                       .to_string()
                       .replace('1', &c.to_string())
                       .replace('0', "."));
            }
            println!("* * * * * * * *");
        }
        print!("\n");
    }
}

