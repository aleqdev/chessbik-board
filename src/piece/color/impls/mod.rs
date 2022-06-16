pub use super::*;

pub mod get_eval;
pub use get_eval::*;

impl PieceColor {
    pub fn opposite(&self) -> Self {
        match self {
            Self::WHITE => Self::BLACK,
            Self::BLACK => Self::WHITE,
        }
    }
}
