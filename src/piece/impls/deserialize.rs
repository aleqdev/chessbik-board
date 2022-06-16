use serde::{Deserialize, Deserializer};

use super::*;

impl<'de> Deserialize<'de> for Piece {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_char(PieceVisitor)
    }
}
