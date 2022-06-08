pub mod impls;
pub use impls::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PieceColor {
    WHITE,
    BLACK,
}
