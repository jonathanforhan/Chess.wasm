pub mod perft;


#[cfg(test)]
#[allow(unused_imports)]
use super::game::{
    *,
    pieces::{Color::*, *},
    notation::*,
    util::*,
};

/* cargo test [TEST NAME] -- --nocapture */
#[test]
fn test_fen() {
    let fen = "r3k3/p1pNqpbr/bn2Pnp1/8/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQq - 0 2";
    let game = fen::decode(fen).unwrap();
    let fen_encoded = fen::encode(&game).unwrap();

    assert_eq!(fen, fen_encoded);
}

#[test]
fn test_move() {
    let game = fen::decode("r3r1k1/p1ppqpb1/bn2Pnp1/4N3/1p2P3/2N2Q1p/PPPBBPPP/R4K1R b - - 0 2").unwrap();
    let (mut w, mut b) = (0, 0);
    for p in &game.pieces {
        match p.color() {
            White => w |= p.bits(),
            Black => b |= p.bits(),
        }
    }

    print_bits(&(w|b), 'o');

    let mv = game.moves();
    for m in &mv {
        if let Pieces::Rook(_) = m {
            print_bits(&m.bits(), 'x');
        }
    }

    println!("{}", mv.len());
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
