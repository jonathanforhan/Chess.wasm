use super::NotationError;

pub fn bits_to_algebraic<'a>(bits: &u128) -> Result<String, NotationError<'a>> {
    for i in (0..128).step_by(16) {
        if bits >> i & 0xff00 != 0 {
            for j in 0..8 {
                if bits >> (i + j + 8) & 1 != 0 {
                    let x = match "abcdefgh".chars().nth(j) {
                        Some(c) => { c.to_string() },
                        None => { return Err(NotationError{ error: "Invalid Notation: a-h error" }) },
                    };
                    let y = (i / 16 + 1).to_string();
                    return Ok(x + &y);
                }
            } // loop j
        }
    } // loop i
    return Err(NotationError { error: "Notation Error: unknown notation" })
}

pub fn algebraic_to_bits<'a>(s: String) -> Result<u128, NotationError<'a>> {
    let x = match "abcdefgh".find(&s[..1]) {
        Some(x) => x,
        None => { return Err(NotationError{ error: "Invalid Notation" }) },
    };
    let y = match s[1..2].parse::<usize>() {
        Ok(y) => y - 1,
        Err(_) => { return Err(NotationError{ error: "Invalid Notation" }) },
    };

    Ok(1 << (y << 4) + 8 + x)
}
