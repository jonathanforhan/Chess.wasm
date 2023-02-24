use super::{
    FenError,
    validate,
};

use crate::{
    Game,
    BitBoard,
    Piece,
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
    EnPassant,
    Color,
};

pub fn decode<'a>(fen: &str) -> Result<Game, FenError<'a>> {
    validate(fen)?;
    let throw = || {
         panic!("Fen Error not caught by fen validation")
    };

    let fen: Vec<&str> = fen.split_whitespace().collect();
    
    let mut pieces = Vec::<BitBoard>::new();

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
                    'p' => { pieces.push(BitBoard::Pawn(Pawn::new(x, y, Color::Black))) },
                    'b' => { pieces.push(BitBoard::Bishop(Bishop::new(x, y, Color::Black))) },
                    'n' => { pieces.push(BitBoard::Knight(Knight::new(x, y, Color::Black))) },
                    'r' => { pieces.push(BitBoard::Rook(Rook::new(x, y, Color::Black))) },
                    'q' => { pieces.push(BitBoard::Queen(Queen::new(x, y, Color::Black))) },
                    'k' => { pieces.push(BitBoard::King(King::new(x, y, Color::Black))) },
                    'P' => { pieces.push(BitBoard::Pawn(Pawn::new(x, y, Color::White))) },
                    'B' => { pieces.push(BitBoard::Bishop(Bishop::new(x, y, Color::White))) },
                    'N' => { pieces.push(BitBoard::Knight(Knight::new(x, y, Color::White))) },
                    'R' => { pieces.push(BitBoard::Rook(Rook::new(x, y, Color::White))) },
                    'Q' => { pieces.push(BitBoard::Queen(Queen::new(x, y, Color::White))) },
                    'K' => { pieces.push(BitBoard::King(King::new(x, y, Color::White))) },
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

    let castling = fen[2].to_string();

    let mut en_passant_square = BitBoard::None;
    let en_passant = fen[3];
    if en_passant != "-" {
        // liberal use of unwrap due to fen validation
        match en_passant.chars().nth(1).unwrap() {
            '3' => {
                const ABC: &str = "abcdefgh";
                let x = ABC.find(en_passant.chars().nth(0).unwrap()).unwrap();
                en_passant_square = BitBoard::EnPassant(EnPassant::new(x, 3, Color::White));
            },
            '6' => {
                const ABC: &str = "abcdefgh";
                let x = ABC.find(en_passant.chars().nth(0).unwrap()).unwrap();
                en_passant_square = BitBoard::EnPassant(EnPassant::new(x, 6, Color::White));
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
