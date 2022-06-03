pub mod ty;
pub use ty::*;

pub mod color;
pub use color::*;

pub mod impls;
pub use impls::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Piece {
    pub ty: PieceTy,
    pub color: PieceColor,
}
