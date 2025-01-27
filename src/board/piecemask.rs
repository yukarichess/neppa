use std::ops::{Index, IndexMut};

use crate::{Bitlist, Piece};

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
