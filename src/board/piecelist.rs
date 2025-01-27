use std::ops::{Index, IndexMut};

use crate::{PieceIndex, Square};

/// A mapping from `PieceIndex` to `Square`.
#[derive(Clone)]
#[repr(transparent)]
pub struct Piecelist([Option<Square>; PieceIndex::COUNT]);

impl Index<PieceIndex> for Piecelist {
    type Output = Option<Square>;

    fn index(&self, index: PieceIndex) -> &Self::Output {
        let index = index.into_inner() as usize;
        &self.0[index]
    }
}

impl IndexMut<PieceIndex> for Piecelist {
    fn index_mut(&mut self, index: PieceIndex) -> &mut Self::Output {
        let index = index.into_inner() as usize;
        &mut self.0[index]
    }
}
