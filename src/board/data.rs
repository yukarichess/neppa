use super::{bitlist::AttackTable, drops::Drops, piece::PromotionlessPiece, pieceindex::PieceIndexArray, piecelist::Piecelist, piecemask::Piecemask, side::Sidemask};

use crate::{Piece, Side, Square};

pub struct BoardData {
    attacks: AttackTable,
    index: PieceIndexArray,
    piecelist: Piecelist,
    piecemask: Piecemask,
    sidemask: Sidemask,
    drops: Drops,
}

impl BoardData {
    pub(super) fn new() -> Self {
        Self {
            attacks: AttackTable::new(),
            index: PieceIndexArray::new(),
            piecelist: Piecelist::new(),
            piecemask: Piecemask::new(),
            sidemask: Sidemask::new(),
            drops: Drops::new(),
        }
    }

    pub fn attacks(&self) -> &AttackTable {
        &self.attacks
    }

    pub fn index(&self) -> &PieceIndexArray {
        &self.index
    }

    pub fn piecelist(&self) -> &Piecelist {
        &self.piecelist
    }

    pub fn piecemask(&self) -> &Piecemask {
        &self.piecemask
    }

    pub fn sidemask(&self) -> &Sidemask {
        &self.sidemask
    }

    pub fn drops(&self) -> &Drops {
        &self.drops
    }

    pub fn add_piece(&mut self, piece: Piece, square: Square, side: Side) {
        let index = self.sidemask.add_piece(side);
        self.index.add_piece(index, square);
        self.piecelist.add_piece(index, square);
        self.piecemask.add_piece(piece, index);
    }

    pub fn add_drop(&mut self, side: Side, piece: PromotionlessPiece, count: u8) {
        self.drops.add_drop(side, piece, count);
    }
}
