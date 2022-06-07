use crate::{IntoPieceDescriptor, PieceDescriptor, PieceOrientation};

use super::*;

impl IntoPieceDescriptor for Piece {
    fn into_piece_descriptor(self) -> PieceDescriptor {
        PieceDescriptor {
            piece: self,
            orientation: PieceOrientation::FORWARD,
        }
    }
}
