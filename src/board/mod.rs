use crate::{Piece, BoardStatus};

pub mod impls;
pub use impls::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(bound = "T: serde::Serialize, for<'de2> T: serde::Deserialize<'de2>")]
pub struct Board<T: serde::Serialize> {
    #[serde(with = "serde_big_array::BigArray")]
    pub cells: [T; 54],
    pub status: BoardStatus
}
