pub mod impls;
pub use impls::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PieceColor {
    WHITE,
    BLACK,
}
