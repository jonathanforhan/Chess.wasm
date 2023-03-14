mod debug;
#[cfg(test)]
pub mod test {
use crate::game::{
    *,
    pieces::*,
};
use std::{
    thread,
    thread::JoinHandle,
    sync::{Arc, Mutex},
};
use super::debug::test::debug;

/* More info on Perft test can be found at:
 * https://www.chessprogramming.org/Perft_Results
 */

/* yarn perft [depth] */
/* yarn perft-threaded [depth] */
/* yarn perft-debug [depth] */
#[test]
pub fn perft() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 5);

    let depth = args[4].parse().unwrap_or_else(|_| panic!("Perft depth required"));
    let opt = args[3].clone();

    assert!(depth < 10);

    let game = fen::decode("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    //let game = fen::decode("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();

    let perft;
    match opt.as_str() {
        "threaded" => perft = gen_nodes_threaded(game, depth, 8),
        "debug" => {
            std::env::set_var("RUST_BACKTRACE", "1");
            debug(game, depth);
            let game = fen::decode("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
            debug(game, depth);
            let game = fen::decode("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
            debug(game, depth);

            /* FAILING */
            //let game = fen::decode("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap();
            //debug(game, depth);
            /* * * * * */

            let game = fen::decode("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
            debug(game, depth);
            let game = fen::decode("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ").unwrap();
            debug(game, depth);
            return;
        },
        _ => perft = gen_nodes(game, depth),
    }

    match depth {
        0 => (),
        1 => assert_eq!(perft, 20),
        2 => assert_eq!(perft, 400),
        3 => assert_eq!(perft, 8_902),
        4 => assert_eq!(perft, 197_281),
        5 => assert_eq!(perft, 4_865_609),
        6 => assert_eq!(perft, 119_060_324),
        7 => assert_eq!(perft, 3_195_901_860),
        8 => assert_eq!(perft, 84_998_978_956),
        9 => assert_eq!(perft, 2_439_530_234_167),
        _ => panic!()
    }

    fn gen_nodes(game: Game, depth: u32) -> usize {
        let moves = game.moves().unwrap();
        if depth <= 1 { return moves.len(); }
        let moves = moves.iter().map(|x| {
            *x.bits()
        }).collect::<Vec<u128>>();
        let mut perft = 0;
        for m in moves {
            let mut game_node = game.clone();
            game_node.move_piece(m);
            perft += gen_nodes(game_node, depth-1);
        }
        return perft;
    }

    fn gen_nodes_threaded(game: Game, depth: u32, threads: usize) -> usize {
        let moves = game.moves().unwrap().iter().map(|x| {
            *x.bits()
        }).collect::<Vec<u128>>();
        if depth <= 1 { return moves.len(); }

        let perft = Arc::new(Mutex::new(0usize));
        let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(8);
        let m = moves.len();
        let n = m / threads;

        for i in 0..threads {
            let game_copy = game.clone();
            let moves_copy = moves.clone();
            let perft_copy = perft.clone();
            let mut move_count = 0usize;
            handles.push(thread::spawn(move || {
                let begin = n * i;
                let mut end = n * (i + 1);
                if i == threads-1 { end = m; }
                for j in begin..end {
                    let mut game_node = game_copy.clone();
                    game_node.move_piece(moves_copy[j]);
                    move_count += gen_nodes(game_node, depth-1);
                }
                *perft_copy.lock().unwrap() += move_count;
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }

        return *perft.lock().unwrap();
    }
}
}
