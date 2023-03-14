mod perft;
#[cfg(test)]
mod test {
#[allow(unused)]
use crate::{game::notation::algebraic_to_bits, engine::Engine};

use super::super::game::{
    fen,
    pieces::{Color::*, *},
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
    //std::env::set_var("RUST_BACKTRACE", "1");
    let game = fen::decode("1n2k2r/r7/p6p/5Np1/Pp2n1P1/8/1PP2QKP/R7 b k - 0 31").unwrap();
    let (mut w, mut b) = (0, 0);
    for p in &game.pieces {
        match p.color() {
            White => w |= p.bits(),
            Black => b |= p.bits(),
        }
    }

    let mv = Engine::best_move("1n2k2r/r7/p6p/5Np1/Pp2n1P1/8/1PP2QKP/R7 b k - 0 31".into()).unwrap();

    print_bits(mv.bits(), 'x');
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
}
