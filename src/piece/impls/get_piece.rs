use crate::GetPiece;

use super::*;

impl GetPiece for Piece {
    fn get_piece(&self) -> Option<&Piece> {
        Some(self)
    }
}
