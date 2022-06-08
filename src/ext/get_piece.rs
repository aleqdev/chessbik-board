use crate::Piece;

pub trait GetPiece {
    fn get_piece(&self) -> Option<&Piece>;
}
