pub mod impls;
pub use impls::*;

use crate::{CubeRotation, PiecePosition};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum PieceMove {
    Slide(PiecePosition),
    Take(PiecePosition),
    Rotation(CubeRotation),
}
