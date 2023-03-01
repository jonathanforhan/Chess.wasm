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
    let game = fen::decode("8/1P6/8/4p3/1k4Kn/8/8/8 w - - 0 1").unwrap();

    let mut r = 0u128;
    let mut x = 0u128;

    for p in &game.pieces {
        x |= p.bits();
    }

    for m in game.moves() {
        r |= m.bits();
    }

    print_bits(&x, 'x');
    print_bits(&r, 'x');
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
