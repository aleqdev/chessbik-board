pub use super::*;

impl PieceMove {
    pub fn slide(pos: impl Into<PiecePosition>) -> Self {
        Self::Slide(pos.into())
    }

    pub fn take(pos: impl Into<PiecePosition>) -> Self {
        Self::Take(pos.into())
    }

    pub fn eq_position(&self, pos: &PiecePosition) -> bool {
        match self {
            Self::Slide(p) | Self::Take(p) => *p == *pos,
            Self::Rotation(..) => false,
        }
    }

    pub fn eq_rotation(&self, rot: &CubeRotation) -> bool {
        match self {
            Self::Slide(..) | Self::Take(..) => false,
            Self::Rotation(r) => *r == *rot,
        }
    }
}
