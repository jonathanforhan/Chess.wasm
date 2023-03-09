#[cfg(test)]
#[allow(unused_imports)]
use crate::game::{
    *,
    pieces::*,
    notation::*,
    util::*,
};

#[test]
pub fn perft() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 4);
    let depth = args[3].parse().unwrap();

    let game = fen::decode( "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let perft = gen_nodes(game, depth);
    match depth {
        0 => (),
        1 => assert_eq!(perft, 20),
        2 => assert_eq!(perft, 400),
        3 => assert_eq!(perft, 8_901),
        4 => assert_eq!(perft, 197_281),
        5 => assert_eq!(perft, 4_865_609),
        6 => assert_eq!(perft, 119_060_324),
        7 => assert_eq!(perft, 3_195_901_860),
        8 => assert_eq!(perft, 84_998_978_956),
        9 => assert_eq!(perft, 2_439_530_234_167),
        _ => panic!("Depth too high")
    }

    fn gen_nodes(game: Game, depth: u32) -> usize {
        let moves = game.moves();
        if depth <= 1 { return moves.len(); }
        let mut perft = 0;
        for m in moves {
            let mut game_node = game.clone();
            game_node.move_piece(*m.bits());
            perft += gen_nodes(game_node, depth-1);
        }
        perft
    }
}
