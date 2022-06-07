use crate::PieceDescriptor;

pub trait IntoPieceDescriptor {
    fn into_piece_descriptor(self) -> PieceDescriptor;
}
