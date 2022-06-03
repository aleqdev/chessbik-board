pub mod impls;
pub use impls::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PieceTy {
    PAWN,
    ROOK,
    KNIGHT,
    BISHOP,
    QUEEN,
    KING,
    MAGE,
}
