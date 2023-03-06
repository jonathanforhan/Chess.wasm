mod perft;


#[cfg(test)]
#[allow(unused_imports)]
use super::game::{
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
    let mut game = fen::decode("rnbqk1nr/ppppppbp/8/6p1/8/1QP5/PP1PPPPP/RNB1KBNR w KQkq - 4 3").unwrap();
    let (mut w, mut b) = (0, 0);
    game.init_boards(&mut w, &mut b);

    print_bits(&(w|b), 'o');

    let src = algebraic_to_bits("b3".into()).unwrap();
    let dst = algebraic_to_bits("f7".into()).unwrap();

    game.move_piece(src | dst);

    let (mut w, mut b) = (0, 0);
    game.init_boards(&mut w, &mut b);
    print_bits(&(w|b), 'o');

    let mv = game.moves();
    for m in mv {
        print_bits(m.bits(), 'x');
    }
}

#[test]
fn test_en_passant() {
    let mut game = fen::decode("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    let src = algebraic_to_bits("a2".into()).unwrap();
    let dst = algebraic_to_bits("a4".into()).unwrap();

    game.move_piece(src | dst);

    let (mut w, mut b) = (0u128, 0u128);

    game.init_boards(&mut w, &mut b);

    print_bits(&(w | b), 'x');
    print_bits(&game.en_passant_square, 'e');
    println!("{}", bits_to_algebraic(&game.en_passant_square).unwrap());
}

pub fn print_bits(x: &u128, c: char) {
    for i in (-15..=112).rev().step_by(16) { // 0..128 but with rev-step
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
