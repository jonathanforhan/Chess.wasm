use crate::game::util::castle::*;
use std::error::Error;

use super::{
    validate,
    super::{
        Game,
        pieces::*,
    }
};

pub fn decode<'a>(fen: &str) -> Result<Game, Box<dyn Error>> {
    validate(fen)?;
    let throw = || {
         panic!("Fen Error not caught by fen validation")
    };

    let fen: Vec<&str> = fen.split_whitespace().collect();
    
    let mut pieces = Vec::<Pieces>::new();

    // decode board
    let rows: Vec<&str> = fen[0].split('/').collect();
    for (i, s) in rows.into_iter().enumerate() {
        let y = 7 - i;
        let mut x: usize = 0;
        for c in s.chars() {
            if let Some(c) = c.to_digit(10) {
                x += c as usize;
            } else {
                match c {
                    'p' => { pieces.push(Pieces::Pawn(Pawn::new(x, y, Color::Black))) },
                    'b' => { pieces.push(Pieces::Bishop(Bishop::new(x, y, Color::Black))) },
                    'n' => { pieces.push(Pieces::Knight(Knight::new(x, y, Color::Black))) },
                    'r' => { pieces.push(Pieces::Rook(Rook::new(x, y, Color::Black))) },
                    'q' => { pieces.push(Pieces::Queen(Queen::new(x, y, Color::Black))) },
                    'k' => { pieces.push(Pieces::King(King::new(x, y, Color::Black))) },
                    'P' => { pieces.push(Pieces::Pawn(Pawn::new(x, y, Color::White))) },
                    'B' => { pieces.push(Pieces::Bishop(Bishop::new(x, y, Color::White))) },
                    'N' => { pieces.push(Pieces::Knight(Knight::new(x, y, Color::White))) },
                    'R' => { pieces.push(Pieces::Rook(Rook::new(x, y, Color::White))) },
                    'Q' => { pieces.push(Pieces::Queen(Queen::new(x, y, Color::White))) },
                    'K' => { pieces.push(Pieces::King(King::new(x, y, Color::White))) },
                    _ => { throw(); }
                }
                x += 1;
            }
        }
    }

    let turn = match fen[1] {
        "w" => Color::White,
        "b" => Color::Black,
        _ => throw()
    };

    let mut castling = 0u16;
    if fen[2].contains('K') { castling += K_ID; }
    if fen[2].contains('Q') { castling += Q_ID; }
    if fen[2].contains('k') { castling += k_ID; }
    if fen[2].contains('q') { castling += q_ID; }

    let mut en_passant_square: u128 = 0;
    let en_passant = fen[3];
    if en_passant != "-" {
        // liberal use of unwrap due to fen validation
        match en_passant.chars().nth(1).unwrap() {
            '3' => {
                const ABC: &str = "abcdefgh";
                let x = ABC.find(en_passant.chars().nth(0).unwrap()).unwrap();
                en_passant_square = 1 << 0x20 + 8 + x;
            },
            '6' => {
                const ABC: &str = "abcdefgh";
                let x = ABC.find(en_passant.chars().nth(0).unwrap()).unwrap();
                en_passant_square = 1 << 0x50 + 8 + x;
            },
            _ => { throw(); }
        }
    }

    let half_moves = fen[4].parse::<u16>().unwrap();
    let move_count = fen[5].parse::<u16>().unwrap();

    Ok(Game::new(
            pieces,
            turn,
            castling,
            en_passant_square,
            half_moves,
            move_count
            ))
}

