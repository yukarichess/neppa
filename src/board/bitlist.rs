use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Index, IndexMut, Not};

use crate::Square;

use super::PieceIndex;

/// A set of 40 bits, each representing a piece.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Bitlist(u64);

impl From<PieceIndex> for Bitlist {
    fn from(index: PieceIndex) -> Self {
        Self(1_u64 << index.into_inner())
    }
}

impl From<u64> for Bitlist {
    fn from(index: u64) -> Self {
        Self(index)
    }
}

impl BitAnd for Bitlist {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitlist {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for Bitlist {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitlist {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl Not for Bitlist {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

/// Array that stores the attacks to a square.
#[derive(Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct AttackTable([Bitlist; Square::COUNT]);

impl Index<Square> for AttackTable {
    type Output = Bitlist;

    fn index(&self, square: Square) -> &Self::Output {
        let square = square.into_inner() as usize;
        &self.0[square]
    }
}

impl IndexMut<Square> for AttackTable {
    fn index_mut(&mut self, square: Square) -> &mut Self::Output {
        let square = square.into_inner() as usize;
        &mut self.0[square]
    }
}
