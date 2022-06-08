pub mod ty;
pub use ty::*;

pub mod color;
pub use color::*;

pub mod impls;
pub use impls::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Piece {
    pub ty: PieceTy,
    pub color: PieceColor,
}
