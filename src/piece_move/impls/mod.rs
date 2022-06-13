pub use super::*;

impl PieceMove {
    pub fn slide(pos: impl Into<PiecePosition>) -> Self {
        Self::Slide(pos.into())
    }

    pub fn take(pos: impl Into<PiecePosition>) -> Self {
        Self::Take(pos.into())
    }

    pub fn eq_position(&self, pos: PiecePosition) -> bool {
        match self {
            Self::Slide(p) => *p == pos,
            Self::Take(p) => *p == pos,
        }
    }
}
