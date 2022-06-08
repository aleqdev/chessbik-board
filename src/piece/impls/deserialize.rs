use serde::de::{Error, Unexpected};

use super::*;

impl<'de> serde::Deserialize<'de> for Piece {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        match char::deserialize(deserializer)? {
            'P' => Ok(Self::WHITE_PAWN),
            'p' => Ok(Self::BLACK_PAWN),
            'R' => Ok(Self::WHITE_ROOK),
            'r' => Ok(Self::BLACK_ROOK),
            'N' => Ok(Self::WHITE_KNIGHT),
            'n' => Ok(Self::BLACK_KNIGHT),
            'B' => Ok(Self::WHITE_BISHOP),
            'b' => Ok(Self::BLACK_BISHOP),
            'Q' => Ok(Self::WHITE_QUEEN),
            'q' => Ok(Self::BLACK_QUEEN),
            'K' => Ok(Self::WHITE_KING),
            'k' => Ok(Self::BLACK_KING),
            'M' => Ok(Self::WHITE_MAGE),
            'm' => Ok(Self::BLACK_MAGE),
            c => Err(Error::invalid_value(Unexpected::Char(c), &"src/piece/impls/serialize"))
        }
    }
}