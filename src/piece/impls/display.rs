use crate::{PieceColor, PieceTy};

use super::*;

impl std::fmt::Display for Piece {
    #[cfg(any(unix, feature = "rich_piece_display"))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.ty, self.color) {
            (PieceTy::PAWN, PieceColor::BLACK) => "♙".fmt(f),
            (PieceTy::ROOK, PieceColor::BLACK) => "♖".fmt(f),
            (PieceTy::KNIGHT, PieceColor::BLACK) => "♘".fmt(f),
            (PieceTy::BISHOP, PieceColor::BLACK) => "♗".fmt(f),
            (PieceTy::QUEEN, PieceColor::BLACK) => "♕".fmt(f),
            (PieceTy::KING, PieceColor::BLACK) => "♔".fmt(f),
            (PieceTy::MAGE, PieceColor::BLACK) => "◇".fmt(f),

            (PieceTy::PAWN, PieceColor::WHITE) => "♟".fmt(f),
            (PieceTy::ROOK, PieceColor::WHITE) => "♜".fmt(f),
            (PieceTy::KNIGHT, PieceColor::WHITE) => "♞".fmt(f),
            (PieceTy::BISHOP, PieceColor::WHITE) => "♝".fmt(f),
            (PieceTy::QUEEN, PieceColor::WHITE) => "♛".fmt(f),
            (PieceTy::KING, PieceColor::WHITE) => "♚".fmt(f),
            (PieceTy::MAGE, PieceColor::WHITE) => "◈".fmt(f),
        }
    }

    #[cfg(not(any(unix, feature = "rich_piece_display")))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.ty, self.color) {
            (PieceTy::PAWN, PieceColor::WHITE) => "P".fmt(f),
            (PieceTy::ROOK, PieceColor::WHITE) => "R".fmt(f),
            (PieceTy::KNIGHT, PieceColor::WHITE) => "N".fmt(f),
            (PieceTy::BISHOP, PieceColor::WHITE) => "B".fmt(f),
            (PieceTy::QUEEN, PieceColor::WHITE) => "Q".fmt(f),
            (PieceTy::KING, PieceColor::WHITE) => "K".fmt(f),
            (PieceTy::MAGE, PieceColor::WHITE) => "M".fmt(f),

            (PieceTy::PAWN, PieceColor::BLACK) => "p".fmt(f),
            (PieceTy::ROOK, PieceColor::BLACK) => "r".fmt(f),
            (PieceTy::KNIGHT, PieceColor::BLACK) => "n".fmt(f),
            (PieceTy::BISHOP, PieceColor::BLACK) => "b".fmt(f),
            (PieceTy::QUEEN, PieceColor::BLACK) => "q".fmt(f),
            (PieceTy::KING, PieceColor::BLACK) => "k".fmt(f),
            (PieceTy::MAGE, PieceColor::BLACK) => "n".fmt(f),
        }
    }
}
