use serde::de::{Error, Visitor, Unexpected};

use super::*;

pub struct PieceVisitor;

pub const EXPECTING: &str = "(P, p, R, r, N, n, B, b, Q, q, K, k, M, m)";

impl<'de> Visitor<'de> for PieceVisitor {
    type Value = Piece;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(EXPECTING)
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: Error, 
    {
        match v {
            'P' => Ok(Piece::WHITE_PAWN),
            'p' => Ok(Piece::BLACK_PAWN),
            'R' => Ok(Piece::WHITE_ROOK),
            'r' => Ok(Piece::BLACK_ROOK),
            'N' => Ok(Piece::WHITE_KNIGHT),
            'n' => Ok(Piece::BLACK_KNIGHT),
            'B' => Ok(Piece::WHITE_BISHOP),
            'b' => Ok(Piece::BLACK_BISHOP),
            'Q' => Ok(Piece::WHITE_QUEEN),
            'q' => Ok(Piece::BLACK_QUEEN),
            'K' => Ok(Piece::WHITE_KING),
            'k' => Ok(Piece::BLACK_KING),
            'M' => Ok(Piece::WHITE_MAGE),
            'm' => Ok(Piece::BLACK_MAGE),
            c => Err(Error::invalid_value(Unexpected::Char(c), &EXPECTING))
        }
    }
}