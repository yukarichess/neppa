use std::fmt::Display;

// this is ordered in lowest-to-highest value, at least partially as a memory aid for me.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Piece {
    /// (歩兵 "foot soldier")
    Pawn,
    /// (香車 "incense chariot")
    Lance,
    /// (桂馬 "cassia horse")
    Knight,
    /// (銀将 "silver general")
    Silver,
    /// (と金 "reaches gold")
    PromotedPawn,
    /// (成香 "promoted incense")
    PromotedLance,
    /// (成桂 "promoted cassia")
    PromotedKnight,
    /// (成銀 "promoted silver")
    PromotedSilver,
    /// (金将 "gold general")
    Gold,
    /// (角行 "angle mover")
    Bishop,
    /// (飛車 "flying chariot")
    Rook,
    /// (竜馬 "dragon horse")
    PromotedBishop,
    /// (竜王 "dragon king")
    PromotedRook,
    /// (王将 "king general" / 玉将 "jeweled general")
    King,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_promoted() {
            write!(f, "+")?;
        }
        write!(f, "{}", PromotionlessPiece::from(*self))
    }
}

impl Piece {
    pub const COUNT: usize = 14;

    pub fn is_promoted(self) -> bool {
        match self {
            Piece::Pawn => false,
            Piece::Lance => false,
            Piece::Knight => false,
            Piece::Silver => false,
            Piece::PromotedPawn => true,
            Piece::PromotedLance => true,
            Piece::PromotedKnight => true,
            Piece::PromotedSilver => true,
            Piece::Gold => false,
            Piece::Bishop => false,
            Piece::Rook => false,
            Piece::PromotedBishop => true,
            Piece::PromotedRook => true,
            Piece::King => false,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum PromotionlessPiece {
    /// (歩兵 "foot soldier")
    Pawn,
    /// (香車 "incense chariot")
    Lance,
    /// (桂馬 "cassia horse")
    Knight,
    /// (銀将 "silver general")
    Silver,
    /// (金将 "gold general")
    Gold,
    /// (角行 "angle mover")
    Bishop,
    /// (飛車 "flying chariot")
    Rook,
    /// (王将 "king general" / 玉将 "jeweled general")
    King,
}

impl From<Piece> for PromotionlessPiece {
    fn from(value: Piece) -> Self {
        match value {
            Piece::Pawn => Self::Pawn,
            Piece::Lance => Self::Lance,
            Piece::Knight => Self::Knight,
            Piece::Silver => Self::Silver,
            Piece::PromotedPawn => Self::Pawn,
            Piece::PromotedLance => Self::Lance,
            Piece::PromotedKnight => Self::Knight,
            Piece::PromotedSilver => Self::Silver,
            Piece::Gold => Self::Gold,
            Piece::Bishop => Self::Bishop,
            Piece::Rook => Self::Rook,
            Piece::PromotedBishop => Self::Bishop,
            Piece::PromotedRook => Self::Rook,
            Piece::King => Self::King,
        }
    }
}

impl Display for PromotionlessPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece = match self {
            PromotionlessPiece::Pawn => 'p',
            PromotionlessPiece::Lance => 'l',
            PromotionlessPiece::Knight => 'n',
            PromotionlessPiece::Silver => 's',
            PromotionlessPiece::Gold => 'g',
            PromotionlessPiece::Bishop => 'b',
            PromotionlessPiece::Rook => 'r',
            PromotionlessPiece::King => 'k',
        };
        write!(f, "{}", piece)
    }
}

impl PromotionlessPiece {
    pub const COUNT: usize = 8;
}
