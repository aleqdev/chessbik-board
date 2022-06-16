use serde::{Serialize, Serializer};

use super::*;

impl Serialize for Piece {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_char(match (self.ty, self.color) {
            (PieceTy::PAWN, PieceColor::WHITE) => 'P',
            (PieceTy::PAWN, PieceColor::BLACK) => 'p',
            (PieceTy::ROOK, PieceColor::WHITE) => 'R',
            (PieceTy::ROOK, PieceColor::BLACK) => 'r',
            (PieceTy::KNIGHT, PieceColor::WHITE) => 'N',
            (PieceTy::KNIGHT, PieceColor::BLACK) => 'n',
            (PieceTy::BISHOP, PieceColor::WHITE) => 'B',
            (PieceTy::BISHOP, PieceColor::BLACK) => 'b',
            (PieceTy::QUEEN, PieceColor::WHITE) => 'Q',
            (PieceTy::QUEEN, PieceColor::BLACK) => 'q',
            (PieceTy::KING, PieceColor::WHITE) => 'K',
            (PieceTy::KING, PieceColor::BLACK) => 'k',
            (PieceTy::MAGE, PieceColor::WHITE) => 'M',
            (PieceTy::MAGE, PieceColor::BLACK) => 'm',
        })
    }
}
