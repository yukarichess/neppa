mod board;

use std::str::FromStr;

pub use crate::board::{Bitlist, Board, Piece, PieceIndex, Side, Square};

fn main() {
    let board = Board::from_str("9/9/9/3k5/9/5K3/9/9/9 b RB2G2S2N2L9Prb2g2s2n2l9p 1").unwrap();
    println!("{board}");
}
