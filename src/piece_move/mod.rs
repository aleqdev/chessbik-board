pub mod ty;
pub use ty::*;

pub mod impls;
pub use impls::*;

use crate::PiecePosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum PieceMove {
    Slide(PiecePosition),
    Take(PiecePosition),
}