pub fn bits_to_algebraic(bits: &u128) -> Option<String> {
    for i in (0..128).step_by(16) {
        if bits >> i & 0xff00 != 0 {
            for j in 0..8 {
                if bits >> (i + j + 8) & 1 != 0 {
                    let x = "abcdefgh".chars().nth(j).unwrap().to_string();
                    let y = (i / 16 + 1).to_string();
                    return Some(x + &y);
                }
            } // loop j
        }
    } // loop i
    None
}
