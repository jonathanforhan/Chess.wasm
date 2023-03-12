use crate::game::util::castle;
use std::error::Error;

use super::{validate, FenError};

use super::super::{
    Game,
    pieces::*,
    notation::bits_to_algebraic
};

pub fn encode<'a>(game: &Game) -> Result<String, Box<dyn Error>> {
    let pos = |bits: u128| {
        for i in (0..128).step_by(16) {
            if bits >> i & 0xff00 != 0 {
                for j in 0..8 {
                    if bits >> (i + j + 8) & 1 != 0 {
                        return Some((j, i/16));
                    }
                }
            }
        }
        None
    };

    let mut board: Vec<Vec<char>> = vec![vec!['.'; 8]; 8];

    for p in game.pieces.iter() {
        let xy = pos(*p.bits()).ok_or_else(|| FenError("Invalid bit boards".into()))?;
        match p {
            Pieces::Pawn(p) => {
                if *p.color() == Color::White { board[xy.1][xy.0] = 'P'; }
                else { board[xy.1][xy.0] = 'p'; }
            },
            Pieces::Knight(n) => {
                if *n.color() == Color::White { board[xy.1][xy.0] = 'N'; }
                else { board[xy.1][xy.0] = 'n'; }
            },
            Pieces::Bishop(b) => {
                if *b.color() == Color::White { board[xy.1][xy.0] = 'B'; }
                else { board[xy.1][xy.0] = 'b'; }
            },
            Pieces::Rook(r) => {
                if *r.color() == Color::White { board[xy.1][xy.0] = 'R'; }
                else { board[xy.1][xy.0] = 'r'; }
            },
            Pieces::Queen(q) => {
                if *q.color() == Color::White { board[xy.1][xy.0] = 'Q'; }
                else { board[xy.1][xy.0] = 'q'; }
            },
            Pieces::King(k) => {
                if *k.color() == Color::White { board[xy.1][xy.0] = 'K'; }
                else { board[xy.1][xy.0] = 'k'; }
            },
        }
    }

    let mut fen = String::new();

    for v in board.iter().rev() {
        let mut empty_count = 0;
        for c in v {
            if *c == '.' {
                empty_count += 1;
                continue;
            }
            if empty_count != 0 {
                fen.push_str(&empty_count.to_string());
                empty_count = 0;
            }
            fen.push(*c);
        }
        if empty_count != 0 {
            fen.push_str(&empty_count.to_string());
        }
        fen.push('/');
    }
    fen.pop();
    fen.push(' ');
    let color = match game.turn {
        Color::White => 'w',
        Color::Black => 'b',
    };
    fen.push(color);
    fen.push(' ');

    let mut castling = String::new();
    if game.castling & castle::K_ID != 0 { castling.push('K'); }
    if game.castling & castle::Q_ID != 0 { castling.push('Q'); }
    if game.castling & castle::k_ID != 0 { castling.push('k'); }
    if game.castling & castle::q_ID != 0 { castling.push('q'); }
    if game.castling == 0 { castling.push('-'); }
    fen.push_str(castling.as_str());

    fen.push(' ');
    if game.en_passant_square != 0 {
        fen.push_str(&bits_to_algebraic(&game.en_passant_square).unwrap_or("-".to_string()));
    } else {
        fen.push('-');
    }
    fen.push(' ');
    fen.push_str(&game.half_moves.to_string());
    fen.push(' ');
    fen.push_str(&game.move_count.to_string());
    validate(&fen).ok();

    Ok(fen)
}
