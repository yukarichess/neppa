use std::ops::{BitOr, Index, IndexMut};

use crate::{Bitlist, Piece};

use super::PieceIndex;

/// A mapping from `Piece` to `Bitlist`.
#[derive(Clone)]
#[repr(transparent)]
pub struct Piecemask([Bitlist; Piece::COUNT]);

impl Index<Piece> for Piecemask {
    type Output = Bitlist;

    fn index(&self, piece: Piece) -> &Self::Output {
        let piece = piece as usize;
        &self.0[piece]
    }
}

impl IndexMut<Piece> for Piecemask {
    fn index_mut(&mut self, piece: Piece) -> &mut Self::Output {
        let piece = piece as usize;
        &mut self.0[piece]
    }
}

impl Piecemask {
    pub(super) fn new() -> Self {
        Self([Bitlist::default(); Piece::COUNT])
    }

    pub fn pieces(&self) -> Bitlist {
        self.0.iter().copied().reduce(BitOr::bitor).unwrap_or(Bitlist::default())
    }

    pub(super) fn add_piece(&mut self, piece: Piece, index: PieceIndex) {

    }
}