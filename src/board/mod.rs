mod bitlist;
mod data;
mod drops;
mod piece;
mod pieceindex;
mod piecelist;
mod piecemask;
mod side;
mod square;

use std::{fmt::Display, str::FromStr};

pub use bitlist::Bitlist;
pub use piece::Piece;
use piece::PromotionlessPiece;
pub use pieceindex::PieceIndex;
pub use side::Side;
pub use square::Square;

pub struct Board {
    data: data::BoardData,
    side: Side,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in (0..=8).rev() {
            let mut accumulator = 0;
            for file in 0..=8 {
                let square = Square::from_rank_file(rank, file).unwrap();
                if let Some(index) = self.data.index()[square] {
                    if accumulator != 0 {
                        write!(f, "{accumulator}")?;
                        accumulator = 0;
                    }
                    let piece = self.data.piecemask().piece_type(index).unwrap();
                    let side = self.data.sidemask().piece_side(index).unwrap();
                    let s = format!("{piece}");
                    write!(f, "{}", if side == Side::Sente { s.to_uppercase() } else { s })?;
                } else {
                    accumulator += 1;
                }
            }
            if accumulator != 0 {
                write!(f, "{accumulator}")?;
            }
            if rank > 0 {
                write!(f, "/")?;
            }
        }

        match self.side {
            Side::Sente => write!(f, " b ")?,
            Side::Gote => write!(f, " w ")?,
        }

        for side in 0..Side::COUNT {
            let side = unsafe { std::mem::transmute::<u8, Side>(side as u8) };
            for promotionless in 0..PromotionlessPiece::COUNT {
                let promotionless = unsafe { std::mem::transmute::<u8, PromotionlessPiece>(promotionless as u8) };
                let drop_count = self.data.drops().drop_count(side, promotionless);
                if drop_count > 0 {
                    if drop_count > 1 {
                        write!(f, "{drop_count}")?;
                    }
                    let s = format!("{promotionless}");
                    write!(f, "{}", if side == Side::Sente { s.to_uppercase() } else { s })?;
                }
            }
        }

        Ok(())
    }
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut b = Self {
            data: data::BoardData::new(),
            side: Side::Sente,
        };

        let mut s = s.chars();

        // Board
        for rank in (0..=8).rev() {
            let mut is_promoted = false;
            let mut file = 0_u8;
            while file <= 8 {
                let c = s.next().unwrap();
                if ('1'..='9').contains(&c) {
                    let inc = u8::from_str(&c.to_string()).unwrap();
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

                    b.data.add_piece(piece, Square::from_rank_file(rank, file).unwrap(), side);

                    is_promoted = false;
                    file += 1;
                }
            }
            if rank > 0 {
                assert_eq!(s.next(), Some('/'));
            }
        }

        {
            let c = s.next().unwrap();
            assert_ne!(c, '[', "xboard FENs aren't supported right now");
            assert_eq!(c, ' ');
        }

        // Side to move
        {
            let c = s.next().unwrap();
            match c {
                'b' => b.side = Side::Sente,
                'w' => b.side = Side::Gote,
                _ => unreachable!("unrecognised side character {c}!"),
            }
            let c = s.next().unwrap();
            assert_eq!(c, ' ');
        }

        // Hand
        {
            let mut c = s.next().unwrap();
            let mut accumulator = 0;

            while c != ' ' {
                if c.is_ascii_digit() {
                    let c = u8::from_str(&c.to_string()).unwrap();
                    accumulator = accumulator * 10 + c;
                } else {
                    let side = if c.is_uppercase() { Side::Sente } else { Side::Gote };
                    let piece = match c.to_ascii_lowercase() {
                        'p' => PromotionlessPiece::Pawn,
                        'l' => PromotionlessPiece::Lance,
                        'n' => PromotionlessPiece::Knight,
                        's' => PromotionlessPiece::Silver,
                        'g' => PromotionlessPiece::Gold,
                        'b' => PromotionlessPiece::Bishop,
                        'r' => PromotionlessPiece::Rook,
                        'k' => PromotionlessPiece::King,
                        _ => unreachable!("unrecognised piece character {c}!"),
                    };

                    if accumulator == 0 {
                        accumulator = 1;
                    }
                    b.data.add_drop(side, piece, accumulator);

                    accumulator = 0;
                }

                c = s.next().unwrap();
            }
        }

        Ok(b)
    }
}