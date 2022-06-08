pub mod ty;
pub use ty::*;

pub mod impls;
pub use impls::*;

use crate::PiecePosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PieceMove {
    pub pos: PiecePosition,
    pub ty: PieceMoveTy,
}
