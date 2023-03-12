use super::{Piece, Pieces, Color};
use crate::{MASK, game::util::promote};

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

    pub fn attacks(&self, attacks: &mut u128) {
        let bits = &self.bits;
        let color = &self.color;

        let mut validate = |test: u128| {
            *attacks |= test & MASK;
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

    fn moves(&self, opp: &u128, team: &u128, moves: &mut Vec<Pieces>) {
        let bits = &self.bits;
        let color = &self.color;

        let validate_attack = |test: u128, moves: &mut Vec<Pieces>| {
            if test & MASK == 0 { return; }
            if test & opp == 0 { return; }
            moves.push(Pieces::Pawn(Pawn { bits: test | bits, color: *color }));
        };

        let validate_move = |test: u128, moves: &mut Vec<Pieces>| -> bool {
            if test & MASK == 0 { return false; }
            if test & team != 0 { return false; }
            if test & opp  != 0 { return false; }
            let mv = Pieces::Pawn(Pawn { bits: test | bits, color: *color});
            promote::add_promotions(&mv , moves);
            moves.push(mv);
            return true;
        };

        match color {
            Color::White => {
                validate_attack(bits << 0x0f, moves);
                validate_attack(bits << 0x11, moves);
                let valid = validate_move(bits << 0x10, moves);
                if valid && bits >> 0x18 & 0xff != 0 {
                    validate_move(bits << 0x20, moves);
                }
            }
            Color::Black => {
                validate_attack(bits >> 0x0f, moves);
                validate_attack(bits >> 0x11, moves);
                let valid = validate_move(bits >> 0x10, moves);
                if valid && bits >> 0x68 & 0xff != 0 {
                    validate_move(bits >> 0x20, moves);
                }
            }
        }
    }

    fn moves_as_bits(&self, opp: &u128, team: &u128, moves: &mut u128) {
        let bits = &self.bits;
        let color = &self.color;

        let validate_attack = |test: u128, moves: &mut u128| {
            if test & MASK == 0 { return; }
            if test & opp == 0 { return; }
            *moves |= test | bits;
        };

        let validate_move = |test: u128, moves: &mut u128| -> bool {
            if test & MASK == 0 { return false; }
            if test & team != 0 { return false; }
            if test & opp  != 0 { return false; }
            *moves |= test | bits;
            return true;
        };

        match color {
            Color::White => {
                validate_attack(bits << 0x0f, moves);
                validate_attack(bits << 0x11, moves);
                let valid = validate_move(bits << 0x10, moves);
                if valid && bits >> 0x18 & 0xff != 0 {
                    validate_move(bits << 0x20, moves);
                }
            }
            Color::Black => {
                validate_attack(bits >> 0x0f, moves);
                validate_attack(bits >> 0x11, moves);
                let valid = validate_move(bits >> 0x10, moves);
                if valid && bits >> 0x68 & 0xff != 0 {
                    validate_move(bits >> 0x20, moves);
                }
            }
        }
    }

    fn moves_as_bits_exclusive(&self, opp: &u128, team: &u128, moves: &mut u128) {
        let bits = &self.bits;
        let color = &self.color;

        let validate_attack = |test: u128, moves: &mut u128| {
            if test & MASK == 0 { return; }
            if test & opp == 0 { return; }
            *moves |= test;
        };

        let validate_move = |test: u128, moves: &mut u128| -> bool {
            if test & MASK == 0 { return false; }
            if test & team != 0 { return false; }
            if test & opp  != 0 { return false; }
            *moves |= test;
            return true;
        };

        match color {
            Color::White => {
                validate_attack(bits << 0x0f, moves);
                validate_attack(bits << 0x11, moves);
                let valid = validate_move(bits << 0x10, moves);
                if valid && bits >> 0x18 & 0xff != 0 {
                    validate_move(bits << 0x20, moves);
                }
            }
            Color::Black => {
                validate_attack(bits >> 0x0f, moves);
                validate_attack(bits >> 0x11, moves);
                let valid = validate_move(bits >> 0x10, moves);
                if valid && bits >> 0x68 & 0xff != 0 {
                    validate_move(bits >> 0x20, moves);
                }
            }
        }
    }
}
