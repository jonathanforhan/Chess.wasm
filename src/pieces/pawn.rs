use super::{Piece, Color};

pub struct Pawn {
    bits: u128,
    color: Color,
}

impl Piece for Pawn {
    type T = Pawn;

    fn new(x: usize, y: usize, color: Color) -> Self::T {
        Pawn { bits: 1 << (y << 4) + 8 + x, color }
    }

    fn bits(&self) -> &u128 {
        &self.bits
    }

    fn moves(&self, opp: &u128, team: &u128) -> Vec<Pawn> {
        let mut valid_moves = Vec::<Pawn>::new();
        let bits = &self.bits;
        let color = &self.color;

        enum Action {
            Attack,
            Move,
        }

        let mut validate = |test: u128, action: Action| -> bool {
            match action {
                Action::Move => {
                    if test & team != 0 { return false; }
                    if test & opp  != 0 { return false; }
                    valid_moves.push(Pawn { bits: test, color: *color});
                    return true;
                }
                Action::Attack => {
                    if test & opp == 0 { return false; }
                    valid_moves.push(Pawn { bits: test, color: *color });
                    return true;
                }
            }
        };

        match color {
            Color::White => {
                validate(bits << 0x0f, Action::Attack);
                validate(bits << 0x11, Action::Attack);
                let valid = validate(bits << 0x10, Action::Move);
                if valid && bits >> 0x18 & 0xff != 0 {
                    validate(bits << 0x20, Action::Move);
                }
            }
            Color::Black => {
                validate(bits >> 0x0f, Action::Attack);
                validate(bits >> 0x11, Action::Attack);
                let valid = validate(bits >> 0x10, Action::Move);
                if valid && bits >> 0x68 & 0xff != 0 {
                    validate(bits >> 0x20, Action::Move);
                }
            }
        }

        return valid_moves;
    }
}
