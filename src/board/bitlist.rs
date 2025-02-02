use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

use crate::{PieceIndex, Square};

/// A set of 40 bits, each representing a piece.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct Bitlist(u64);

impl From<PieceIndex> for Bitlist {
    fn from(index: PieceIndex) -> Self {
        Self(1_u64 << index.into_inner())
    }
}

impl From<u64> for Bitlist {
    fn from(value: u64) -> Self {
        Self(value & 0xFF_FFFF_FFFF)
    }
}

impl From<Bitlist> for u64 {
    fn from(value: Bitlist) -> Self {
        value.0
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

impl Bitlist {
    /// Return the lowest set bit of a `Bitlist` as a `PieceIndex`.
    pub const unsafe fn peek_nonzero(self) -> PieceIndex {
        if self.0 == 0 {
            unsafe { std::hint::unreachable_unchecked() };
        }
        #[allow(clippy::cast_possible_truncation)]
        let bit = self.0.trailing_zeros() as u8;
        PieceIndex::new_unchecked(bit)
    }

}

/// Array that stores the attacks to a square.
/// 
/// This represents the 40-bit attacks as 4+1 bytes.
#[derive(Clone, PartialEq, Eq)]
pub struct AttackTable([u32; Square::COUNT], [u8; Square::COUNT]);

impl AttackTable {
    pub(super) fn new() -> Self {
        Self([0; Square::COUNT], [0; Square::COUNT])
    }

    pub fn get(&self, square: Square) -> Bitlist {
        let square = usize::from(square);
        Bitlist::from((self.0[square] as u64) | ((self.1[square] as u64) << 32))
    }

    pub fn set(&mut self, square: Square, bitlist: Bitlist) {
        let square = usize::from(square);
        let bitlist = u64::from(bitlist);
        self.0[square] = bitlist as u32;
        self.1[square] = (bitlist >> 32) as u8;
    }
}
