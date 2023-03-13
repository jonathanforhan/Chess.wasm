use crate::game::{
    Game,
    pieces::{
        Color::White,
        Color::Black,
        Piece,
        Pieces,
    },
};

pub mod core;
pub mod castle;
pub mod en_passant;
pub mod promote;
pub mod check;

pub struct GameInfo<'a> {
    pub valid_moves: u16,
    pub king: &'a Pieces,
    pub check: bool,
    pub team_pieces: u128,
    pub opp_pieces: u128,
    pub opp_attacks: u128,

    /* opp directional used for pin detection */
    pub opp_diagonal: u128, // bishop and queen
    pub opp_straight: u128, // rook and queen
}

impl<'a> GameInfo<'a> {
    #[inline]
    pub fn init(game: &'a Game) -> Self {
        let mut king: Option<&'a Pieces> = None;
        let check = false;
        let (mut team_pieces, mut opp_pieces) = (0u128, 0u128);
        let opp_attacks = 0u128;
        let (opp_diagonal, opp_straight) = (0u128, 0u128);

        for piece in &game.pieces {
            match (game.turn, piece.color()) {
                (White, White) | (Black, Black) => {
                    if let Pieces::King(_) = piece {
                        king = Some(piece);
                    }
                    team_pieces |= piece.bits()
                },
                (White, Black) | (Black, White) => {
                    opp_pieces |= piece.bits()
                },
            }
        }

        #[allow(unused_unsafe)]
        unsafe {
            GameInfo {
                valid_moves: 0,
                king: king.unwrap(),
                check,
                team_pieces,
                opp_pieces,
                opp_attacks,
                opp_diagonal,
                opp_straight
            }
        }
    }
}
