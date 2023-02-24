use super::*;

#[cfg(test)]
mod tests {
    use wasm_bindgen::UnwrapThrowExt;

    use super::*;
    /* cargo test [TEST NAME] -- --nocapture */
    #[test]
    fn test_fen() {
        let game = fen::decode("8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50").unwrap();
        let mut white_board: u128 = 0;
        let mut black_board: u128 = 0;
        for x in game.pieces  {
            match x.color().unwrap_throw() {
                Color::White => white_board ^= x.bits().unwrap(),
                Color::Black => black_board ^= x.bits().unwrap(),
            }
        }
        print_bits(&white_board, 'w');
        print_bits(&black_board, 'b');
    }

    #[test]
    fn test_moves() {
        let game = fen::decode("8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50").unwrap();
        let moves = game.moves();

        for m in &moves {
            match (m, m.color().unwrap()) {
                (BitBoard::Pawn(m), Color::White) => print_bits(m.bits(), 'P'),
                (BitBoard::Bishop(m), Color::White) => print_bits(m.bits(), 'B'),
                (BitBoard::Knight(m), Color::White) => print_bits(m.bits(), 'N'),
                (BitBoard::Rook(m), Color::White) => print_bits(m.bits(), 'R'),
                (BitBoard::Queen(m), Color::White) => print_bits(m.bits(), 'Q'),
                (BitBoard::King(m), Color::White) => print_bits(m.bits(), 'K'),
                (BitBoard::Pawn(m), Color::Black) => print_bits(m.bits(), 'p'),
                (BitBoard::Bishop(m), Color::Black) => print_bits(m.bits(), 'b'),
                (BitBoard::Knight(m), Color::Black) => print_bits(m.bits(), 'n'),
                (BitBoard::Rook(m), Color::Black) => print_bits(m.bits(), 'r'),
                (BitBoard::Queen(m), Color::Black) => print_bits(m.bits(), 'q'),
                (BitBoard::King(m), Color::Black) => print_bits(m.bits(), 'k'),
                _ => panic!("TEST ERROR")
            }
        }
        assert_eq!(moves.len(), 9);
    }

    fn print_bits(x: &u128, c: char) {
        for i in (-15..=112).rev().step_by(16) { // 0..128 but with rev-step
        //for i in (0..128).step_by(16) {
            //for j in (0..8).rev() {
            for j in 0..8 {
                print!("{} ", (x >> (i + j + 8) & 1)
                       .to_string()
                       .replace('1', &c.to_string())
                       .replace('0', "."));
            }
            //println!("* * * * * * * *");
            print!("\n");
        }
        println!("{}", bits_to_algebraic(x).unwrap());
        print!("\n");
    }
}

