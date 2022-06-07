use crate::Piece;

pub mod impls;
pub use impls::*;

pub struct Board<T> {
    pub cells: [T; 9 * 6],
}
