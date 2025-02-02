use crate::{Bitlist, PieceIndex};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Side {
    /// player who goes first
    Sente,
    /// player who goes second
    Gote,
}

impl Side {
    pub const COUNT: usize = 2;
}

/// A mapping from `Side` to `Bitlist`.
#[derive(Clone)]
#[repr(transparent)]
pub struct Sidemask([Bitlist; Side::COUNT]);

impl Sidemask {
    pub(super) fn new() -> Self {
        Self([Bitlist::default(); Side::COUNT])
    }

    pub fn occupied(&self) -> Bitlist {
        self.0[0] | self.0[1]
    }

    pub fn empty(&self) -> Bitlist {
        !self.occupied()
    }

    pub fn piece_side(&self, index: PieceIndex) -> Option<Side> {
        for piece_side in 0..Side::COUNT {
            if !(self.0[piece_side] & index.into()).empty() {
                return Some(unsafe { std::mem::transmute::<u8, Side>(piece_side as u8) });
            }
        }
        None
    }

    pub(super) fn add_piece(&mut self, side: Side) -> PieceIndex {
        let piece_index = unsafe { self.empty().peek_nonzero() };
        match side {
            Side::Sente => self.0[0] |= piece_index.into(),
            Side::Gote => self.0[1] |= piece_index.into(),
        }
        piece_index
    }
}