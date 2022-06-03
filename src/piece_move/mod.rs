pub mod ty;
pub use ty::*;

pub mod impls;
pub use impls::*;

use crate::PiecePosition;

pub struct PieceMove {
    pub pos: PiecePosition,
    pub ty: PieceMoveTy,
}
