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

impl Piece {
    pub const COUNT: usize = 14;
}
