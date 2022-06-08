use crate::Piece;

pub mod impls;
pub use impls::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Board<T> {
    pub cells: [T; 9 * 6],
}
