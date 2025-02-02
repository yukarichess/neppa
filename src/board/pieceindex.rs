use std::{
    num::NonZeroU8,
    ops::{Index, IndexMut},
};

use crate::Square;

#[allow(clippy::module_name_repetitions)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
#[repr(transparent)]
pub struct PieceIndex(NonZeroU8);

impl PieceIndex {
    pub const COUNT: usize = 40;

    pub fn into_inner(self) -> u8 {
        u8::from(self.0) - 1
    }

    /// # Safety
    /// `x` must be in the range 0-39.
    #[must_use]
    pub const unsafe fn new_unchecked(x: u8) -> Self {
        Self(unsafe { NonZeroU8::new_unchecked(x + 1) })
    }
}

/// A `Square` -> `PieceIndex` mapping.
#[derive(Clone)]
#[repr(transparent)]
pub struct PieceIndexArray([Option<PieceIndex>; Square::COUNT]);

impl Index<Square> for PieceIndexArray {
    type Output = Option<PieceIndex>;

    fn index(&self, square: Square) -> &Self::Output {
        &self.0[usize::from(square)]
    }
}

impl IndexMut<Square> for PieceIndexArray {
    fn index_mut(&mut self, square: Square) -> &mut Self::Output {
        &mut self.0[usize::from(square)]
    }
}

impl PieceIndexArray {
    pub(super) fn new() -> Self {
        Self([None; Square::COUNT])
    }
}
