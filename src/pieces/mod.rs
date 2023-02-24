mod pawn;
mod bishop;
mod knight;
mod rook;
mod queen;
mod king;
mod en_passant;

pub use {
    pawn::Pawn,
    bishop::Bishop,
    knight::Knight,
    rook::Rook,
    queen::Queen,
    king::King,
    en_passant::EnPassant,
};

pub enum BitBoard {
    Pawn(Pawn),
    Bishop(Bishop),
    Knight(Knight),
    Rook(Rook),
    Queen(Queen),
    King(King),
    EnPassant(EnPassant),
    None,
}

// implimentation gets a little messy because
// we have technically unique structs and must match them
impl BitBoard {
    pub fn convert_to_64bit(bits: &u128) -> u64 {
        let mut result: u64 = 0;
        for i in 0..8 {
            result |= (((bits >> (i << 4) + 8) & 0x00ff) << (i << 3)) as u64;
        }
        return result;
    }

    pub fn bits(&self) -> Option<&u128> {
        match &self {
            BitBoard::Pawn(p) =>   Some(p.bits()),
            BitBoard::Bishop(b) => Some(b.bits()),
            BitBoard::Knight(k) => Some(k.bits()),
            BitBoard::Rook(r) =>   Some(r.bits()),
            BitBoard::Queen(q) =>  Some(q.bits()),
            BitBoard::King(k) =>   Some(k.bits()),
            BitBoard::EnPassant(ep) => Some(ep.bits()),
            _ => None
        }
    }

    pub fn color(&self) -> Option<&Color> {
        match &self {
            BitBoard::Pawn(p) =>   Some(p.color()),
            BitBoard::Bishop(b) => Some(b.color()),
            BitBoard::Knight(k) => Some(k.color()),
            BitBoard::Rook(r) =>   Some(r.color()),
            BitBoard::Queen(q) =>  Some(q.color()),
            BitBoard::King(k) =>   Some(k.color()),
            BitBoard::EnPassant(ep) => Some(ep.color()),
            _ => None
        }
    }

    pub fn moves(&self, opp: &u128, team: &u128) -> Option<Vec<BitBoard>> {
        match &self {
            BitBoard::Pawn(p) =>   {
                let result = p.moves(&opp, &team).into_iter().map(|x| BitBoard::Pawn(x)).collect();
                Some(result)
            }
            BitBoard::Bishop(p) =>   {
                let result = p.moves(&opp, &team).into_iter().map(|x| BitBoard::Bishop(x)).collect();
                Some(result)
            }
            BitBoard::Knight(p) =>   {
                let result = p.moves(&opp, &team).into_iter().map(|x| BitBoard::Knight(x)).collect();
                Some(result)
            }
            BitBoard::Rook(p) =>   {
                let result = p.moves(&opp, &team).into_iter().map(|x| BitBoard::Rook(x)).collect();
                Some(result)
            }
            BitBoard::Queen(p) =>   {
                let result = p.moves(&opp, &team).into_iter().map(|x| BitBoard::Queen(x)).collect();
                Some(result)
            }
            BitBoard::King(p) =>   {
                let result = p.moves(&opp, &team).into_iter().map(|x| BitBoard::King(x)).collect();
                Some(result)
            }
            _ => None
        }
    }
}

#[derive(Clone, Copy)]
pub enum Color {
    White,
    Black,
}

// implimented by enum params //
pub trait Piece {
    type T: Piece;
    fn new(x: usize, y: usize, color: Color) -> Self::T;
    fn bits(&self) -> &u128;
    fn color(&self) -> &Color;
    fn moves(&self, opp: &u128, team: &u128) -> Vec<Self::T>;
}
