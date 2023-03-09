use super::{Piece, Pieces, Color};
use crate::MASK;

#[derive(Clone)]
pub struct Pawn {
    bits: u128,
    color: Color,
}

impl Pawn {
    pub fn new(x: usize, y: usize, color: Color) -> Self {
        Pawn { bits: 1 << (y << 4) + 8 + x, color }
    }

    pub fn from_bits(bits: u128, color: Color) -> Self {
        Pawn { bits, color }
    }

    pub fn attacks(&self) -> Vec<Pieces> {
        let mut attacks: Vec<Pieces> = Vec::new();
        let bits = &self.bits;
        let color = &self.color;

        let mut validate = |test: u128| {
            if test & MASK == 0 { return; }
            attacks.push(Pieces::Pawn(Pawn { bits: test, color: *color}));
        };

        match color {
            Color::White => {
                validate(bits << 0x0f);
                validate(bits << 0x11);
            }
            Color::Black => {
                validate(bits >> 0x0f);
                validate(bits >> 0x11);
            }
        }

        return attacks;
    }
}

impl Piece for Pawn {
    fn bits(&self) -> &u128 {
        &self.bits
    }

    fn set_bits(&mut self, bits: &u128) {
        self.bits = *bits;
    }

    fn color(&self) -> &Color {
        &self.color
    }

    fn moves(&self, opp: &u128, team: &u128) -> Vec<Pieces> {
        let mut valid_moves = Vec::<Pieces>::new();
        let bits = &self.bits;
        let color = &self.color;

        enum Action {
            Attack,
            Move,
        }

        let mut validate = |test: u128, action: Action| -> bool {
            match action {
                Action::Move => {
                    if test & MASK == 0 { return false; }
                    if test & team != 0 { return false; }
                    if test & opp  != 0 { return false; }
                    valid_moves.push(Pieces::Pawn(Pawn { bits: test | bits, color: *color}));
                    return true;
                }
                Action::Attack => {
                    if test & MASK == 0 { return false; }
                    if test & opp == 0 { return false; }
                    valid_moves.push(Pieces::Pawn(Pawn { bits: test | bits, color: *color }));
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
