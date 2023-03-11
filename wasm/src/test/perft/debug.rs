/* Using chess crate from 'https://crates.io/crates/chess'
 * for debug this engine
 */

use crate::game::{
    *,
    pieces::{*, Color::*},
    notation::bits_to_algebraic,
    util::*,
};
use std::{
    str::FromStr,
    fs::File,
    io::Write
};
use chess::{
    MoveGen,
    Board,
};

fn uci_moves(game: &Game) -> Vec<String> {
    let mut current = 0u128;
    for p in &game.pieces {
        if p.color() == &game.turn { current |= p.bits(); }
    }

    let mut moves: Vec<String> = Vec::new();

    for mut m in game.moves() {
        match *m.bits() {
            castle::K_ZONE => m.set_bits(&castle::K_MOVE),
            castle::Q_ZONE => m.set_bits(&castle::Q_MOVE),
            castle::k_ZONE => m.set_bits(&castle::k_MOVE),
            castle::q_ZONE => m.set_bits(&castle::q_MOVE),
            _ => ()
        }

        let mut src = current & m.bits();  // find the matching starting location
        let mut dst = m.bits() & !src; // subtract starting pos from move map
        let mut promotion = 0u128;
        let mut pawn = false;
        if let Pieces::Pawn(_) = m {
            pawn = true;
            match m.color() {
                Color::White => {
                    dst &= !promote::BLACK_BACK_RANK;
                    src &= !promote::BLACK_BACK_RANK;
                },
                Color::Black => {
                    dst &= !promote::WHITE_BACK_RANK;
                    src &= !promote::WHITE_BACK_RANK;
                },
            }
            promotion |= m.bits() ^ (src | dst);
        }

        let mut promotion = match promotion {
            promote::WHITE_ROOK => "r",
            promote::WHITE_BISHOP => "b",
            promote::WHITE_KNIGHT => "n",
            promote::BLACK_ROOK => "r",
            promote::BLACK_BISHOP => "b",
            promote::BLACK_KNIGHT => "n",
            _ => ""
        };

        // Convert bits to string
        let from = bits_to_algebraic(&src).unwrap();
        let to = bits_to_algebraic(&dst).unwrap();
        if pawn && promotion.is_empty() {
            if game.turn == White && to.contains("8") {
                promotion = "q";
            }
            else if game.turn == Black && to.contains("1") {
                promotion = "q";
            }
        }
        moves.push(from + &*to + promotion);
    }

    return moves;
}

#[allow(dead_code)]
pub fn debug(game: Game, depth: u32) {
    if depth < 1 { return; }
    let fen = fen::encode(&game).unwrap();
    let board = Board::from_str(&*fen).expect("Valid Fen");

    let moves = game.moves();
    let valid_moves = MoveGen::new_legal(&board);
    let test_moves = uci_moves(&game);

    let perft_error = || {
        let _moves = MoveGen::new_legal(&board);
        let _moves = _moves.map(|x| {
            x.to_string()
        }).collect::<Vec<String>>();
        let mut error_message =
            String::from(format!("ERROR\n\nFen error at node: {}\n\nExpected moves:\n\n", fen));
        for m in &_moves {
            error_message.push_str(&*format!("\t{}\n", &*m));
        }
        error_message.push_str("\n\nMoves generated:\n\n");
        for m in &test_moves {
            error_message.push_str(&*format!("\t{}\n", &*m));
        }

        let mut bad_moves = String::new();
        for m in &_moves {
            if !test_moves.contains(&m) {
                bad_moves.push_str(&*format!("\t{}\n", &*m));
            }
        }
        for m in &test_moves {
            if !_moves.contains(&m) {
                bad_moves.push_str(&*format!("\t{}\n", &*m));
            }
        }

        error_message.push_str(&*format!("\n\nBad moves:\n\n{}", bad_moves));

        let mut f = File::create("./src/test/perft/error.log").expect("File creation error");
        f.write_all(error_message.as_bytes()).expect("Write error");

        panic!("See error.log");
    };
    if valid_moves.len() != test_moves.len() {
        perft_error();
    }
    for m in valid_moves.into_iter() {
        if !test_moves.contains(&m.to_string()) {
            perft_error();
        }
    }

    for m in moves {
        let mut game_copy = game.clone();
        game_copy.move_piece(*m.bits());
        debug(game_copy, depth-1);
    }
}
