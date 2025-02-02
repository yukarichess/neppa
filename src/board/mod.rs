mod bitlist;
mod data;
mod drops;
mod piece;
mod pieceindex;
mod piecelist;
mod piecemask;
mod side;
mod square;

use std::str::FromStr;

pub use bitlist::Bitlist;
pub use piece::Piece;
pub use pieceindex::PieceIndex;
pub use side::Side;
pub use square::Square;

pub struct Board {
    data: data::BoardData,
    side: Side,
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut b = Self {
            data: data::BoardData::new(),
            side: Side::Sente,
        };

        let mut s = s.chars();
        let mut accumulator = 0;
        let mut is_promoted = false;

        for rank in (0..=8).rev() {
            let mut file = 0;
            while file <= 8 {
                let c = s.next().unwrap();
                if ('1'..='9').contains(&c) {
                    let inc = i32::from_str(&c.to_string()).unwrap();
                    file += inc;
                } else if c == '+' {
                    is_promoted = true;
                } else {
                    let side = if c.is_uppercase() { Side::Sente } else { Side::Gote };
                    let piece = match (c.to_ascii_lowercase(), is_promoted) {
                        ('p', false) => Piece::Pawn,
                        ('p', true) => Piece::PromotedPawn,
                        ('l', false) => Piece::Lance,
                        ('l', true) => Piece::PromotedLance,
                        ('n', false) => Piece::Knight,
                        ('n', true) => Piece::PromotedKnight,
                        ('s', false) => Piece::Silver,
                        ('s', true) => Piece::PromotedSilver,
                        ('g', false) => Piece::Gold,
                        ('g', true) => unreachable!("no such thing as a promoted gold"),
                        ('b', false) => Piece::Bishop,
                        ('b', true) => Piece::PromotedBishop,
                        ('r', false) => Piece::Rook,
                        ('r', true) => Piece::PromotedRook,
                        ('k', false) => Piece::King,
                        ('k', true) => unreachable!("no such thing as a promoted king"),
                        _ => unreachable!("unrecognised piece character {c}!"),
                    };

                    b.data.add_piece(piece, side);

                    is_promoted = false;
                    file += 1;
                }
            }
        }

        Ok(b)
    }
}