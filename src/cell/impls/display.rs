use super::*;

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.piece {
            Some(piece) => piece.fmt(f),
            None => " ".fmt(f),
        }
    }
}
