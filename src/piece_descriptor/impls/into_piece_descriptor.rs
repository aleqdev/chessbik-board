use crate::IntoPieceDescriptor;

use super::*;

impl IntoPieceDescriptor for PieceDescriptor {
    fn into_piece_descriptor(self) -> PieceDescriptor {
        self
    }
}
