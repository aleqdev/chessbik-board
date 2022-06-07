use super::*;

impl std::fmt::Display for PieceDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.piece.fmt(f)
    }
}
