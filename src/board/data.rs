use super::{bitlist::AttackTable, pieceindex::PieceIndexArray, piecelist::Piecelist, piecemask::Piecemask};

trait Update {}

pub struct BoardData {
    attacks: AttackTable,
    index: PieceIndexArray,
    piecelist: Piecelist,
    piecemask: Piecemask,
}
