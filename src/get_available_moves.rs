use crate::{PieceMove, PiecePosition};

pub trait GetAvailableMoves {
    fn get_available_moves(&self, pos: impl Into<PiecePosition>) -> Vec<PieceMove>;
}
