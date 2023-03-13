/* Criteria and regex taken from Chess.js source code
 * https://github.com/jhlywa/chess.js/blob/master/src/chess.ts
 */

use std::error::Error;

use regex::Regex;
use super::FenError;

pub fn validate<'a>(fen: &str) -> Result<(), Box<dyn Error>> {
    // 1st requirement: 6 space-seperated fields
    let fen: Vec<&str> = fen.split_whitespace().collect();
    if fen.len() != 6 {
        return Err(Box::new(FenError("Invalid Fen: must contain 6 space-seperated fields".into())));
    }

    // 2nd requirement: move number must be unsigned int
    let _ = fen[5].parse::<u32>().map_err(|_| {
        return Err::<(), Box<FenError>>(Box::new(FenError("Invalid Fen: move number must be positive integer".into())));
    });

    // 3rd requirement: half move counter must be unsigned int
    let _ = fen[4].parse::<u32>().map_err(|_| {
        return Err::<(), Box<FenError>>(Box::new(FenError("Invalid Fen: half move counter must be positive integer".into())));
    });

    // 4th requirement: 4th field is valid en passant square
    let en_passant = Regex::new(r"^(-|[a-h][36])$").unwrap();
    if !en_passant.is_match(fen[3]) {
        return Err(Box::new(FenError("Invalid Fen: invalid en-passant square".into())));
    }

    // 5th requirement: 3rd field is valid castle string
    let castle = Regex::new(r"[^kKqQ-]").unwrap();
    if castle.is_match(fen[2]) {
        return Err(Box::new(FenError("Invalid Fen: invalid castling availability".into())));
    }

    // 6th requirement: 2nd field color
    let color = Regex::new(r"^(w|b)$").unwrap();
    if !color.is_match(fen[1]) {
        return Err(Box::new(FenError("Invalid Fen: side-to-move is invalid".into())));
    }

    // 7th requirement: 1st field contains 8 rows
    let rows: Vec<&str> = fen[0].split('/').collect();
    if rows.len() != 8 {
        return Err(Box::new(FenError("Invalid Fen: piece data does not contain 8 '/'-seperated rows".into())));
    }

    // 8th requirement: every row valid
    for s in rows {
        // check sum of 8 and no numbers in succession
        let mut sum = 0;
        let mut prev_num = false;
        for c in s.chars() {
            if let Some(c) = c.to_digit(10) {
                if prev_num {
                    return Err(Box::new(FenError("Invalid Fen: piece data is invalid, consecutive number".into())));
                }
                sum += c;
                prev_num = true;
            } else {
                let re = Regex::new(r"^[prnbqkPRNBQK]$").unwrap();
                if !re.is_match(&c.to_string()) {
                    return Err(Box::new(FenError("Invalid Fen: piece data is invalid, invalid piece".into())));
                }
                sum += 1;
                prev_num = false;
            }
        }
        if sum != 8 {
            return Err(Box::new(FenError("Invalid Fen: piece data is invalid, wrong number of squares".into())));
        }
    }

    // Ensure coherence between fields
    if fen[3] == "-" {}
    else if fen[3].chars().nth(1).unwrap() == '3' && fen[1] == "w" ||
       fen[3].chars().nth(1).unwrap() == '6' && fen[1] == "b" {
        return Err(Box::new(FenError("Invalid Fen: invalid en-passant square".into())));
    }

    let count_kings = |c: char| {
        // ensure we find a king
        if let Some(i) = fen[0].find(c) {
            let (_, substr) = fen[0].split_at(i + 1);
            // ensure there's only one
            if substr.find(c) != None {
                return Err(Box::new(FenError("Invalid Fen: too many kings".into())));
            }
            Ok(())
        } else {
            return Err(Box::new(FenError("Invalid Fen: too few kings".into())));
        }
    };
    count_kings('K')?;
    count_kings('k')?;

    Ok(())
}
