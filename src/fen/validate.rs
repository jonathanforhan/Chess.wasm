use regex::Regex;
use super::FenError;

pub fn validate<'a>(fen: &str) -> Result<(), FenError<'a>> {
    // Criteria and regex taken from Chess.js source code //
    // 1st requirement: 6 space-seperated fields
    let fen: Vec<&str> = fen.split_whitespace().collect();
    if fen.len() != 6 {
        return Err(FenError { error: "Invalid Fen: must contain 6 space-seperated fields" });
    }

    // 2nd requirement: move number must be unsigned int
    let _ = fen[5].parse::<u32>().map_err(|_| {
        return FenError { error: "Invalid Fen: move number must be positive integer" }
    });

    // 3rd requirement: half move counter must be unsigned int
    let _ = fen[4].parse::<u32>().map_err(|_| {
        return FenError { error: "Invalid Fen: half move counter must be positive integer" }
    });

    // 4th requirement: 4th field is valid en passant square
    let en_passant = Regex::new(r"^(-|[a-h][36])$").unwrap();
    if !en_passant.is_match(fen[3]) {
        return Err(FenError { error: "Invalid Fen: invalid en-passant square" });
    }

    // 5th requirement: 3rd field is valid castle string
    let castle = Regex::new(r"[^kKqQ-]").unwrap();
    if castle.is_match(fen[2]) {
        return Err(FenError { error: "Invalid Fen: invalid castling availability" });
    }

    // 6th requirement: 2nd field color
    let color = Regex::new(r"^(w|b)$").unwrap();
    if !color.is_match(fen[1]) {
        return Err(FenError { error: "Invalid Fen: side-to-move is invalid" });
    }

    // 7th requirement: 1st field contains 8 rows
    let rows: Vec<&str> = fen[0].split('/').collect();
    if rows.len() != 8 {
        return Err(FenError { error: "Invalid Fen: piece data does not contain 8 '/'-seperated rows" });
    }

    // 8th requirement: every row valid
    /* TODO */

    Ok(())
}
