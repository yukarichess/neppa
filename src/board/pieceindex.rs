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
}

/// A `Square` -> `PieceIndex` mapping.
#[derive(Clone)]
#[repr(transparent)]
pub struct PieceIndexArray([Option<PieceIndex>; Square::COUNT]);

impl Index<Square> for PieceIndexArray {
    type Output = Option<PieceIndex>;

    fn index(&self, square: Square) -> &Self::Output {
        let square = square.into_inner() as usize;
        &self.0[square]
    }
}

impl IndexMut<Square> for PieceIndexArray {
    fn index_mut(&mut self, square: Square) -> &mut Self::Output {
        let square = square.into_inner() as usize;
        &mut self.0[square]
    }
}
