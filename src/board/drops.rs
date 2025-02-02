use super::{piece::PromotionlessPiece, Side};

#[repr(transparent)]
pub struct Drops([[u8; PromotionlessPiece::COUNT]; Side::COUNT]);

impl Drops {
    pub(super) fn new() -> Self {
        Self([[0; PromotionlessPiece::COUNT]; Side::COUNT])
    }
}