use crate::{Piece, PieceOrientation};

pub mod impls;
pub use impls::*;

#[derive(Clone, Copy)]
pub struct PieceDescriptor {
    pub piece: Piece,
    pub orientation: PieceOrientation,
}
