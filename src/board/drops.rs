use super::{piece::PromotionlessPiece, Side};

#[repr(transparent)]
pub struct Drops([[u8; PromotionlessPiece::COUNT]; Side::COUNT]);

impl Drops {
    pub(super) fn new() -> Self {
        Self([[0; PromotionlessPiece::COUNT]; Side::COUNT])
    }

    pub fn add_drop(&mut self, side: Side, piece: PromotionlessPiece, count: u8) {
        let side = side as usize;
        let piece = piece as usize;
        self.0[side][piece] += count;
    }

    pub fn drop_count(&self, side: Side, piece: PromotionlessPiece) -> u8 {
        let side = side as usize;
        let piece = piece as usize;
        self.0[side][piece]
    }
}
