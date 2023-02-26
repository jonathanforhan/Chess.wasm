mod perft;

#[cfg(test)]
mod tests {
    use super::super::game::{
        *,
        pieces::*,
        notation::*,
    };

    /* cargo test [TEST NAME] -- --nocapture */
    #[test]
    fn test_fen() {
        let game = fen::decode("8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50").unwrap();
        let fen = fen::encode(&game).unwrap();
        //println!("{}", fen);
        assert_eq!(fen, "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50".to_string());
    }

    #[test]
    fn test_moves() {
        let mut game = fen::decode("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

        let mut board: u128 = 0;

        for p in &game.pieces {
            board |= p.bits();
        }

        print_bits(&board, 'x');

        let src = algebraic_to_bits("a2".to_string()).unwrap();
        let dst = algebraic_to_bits("a3".to_string()).unwrap();

        if let Ok(_) = game.valid_move(&(src | dst)) {
            game.move_piece(src | dst);
        } else {
            panic!("Invalid Move")
        }

        board = 0;

        for p in &game.pieces {
            board |= p.bits();
        }

        print_bits(&board, 'x');
        game.turn = Color::Black;

        let src = algebraic_to_bits("g8".to_string()).unwrap();
        let dst = algebraic_to_bits("f6".to_string()).unwrap();

        game.move_piece(src | dst);

        board = 0;

        for p in game.pieces {
            board |= p.bits();
        }

        print_bits(&board, 'x');
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
        print!("\n");
    }
}
