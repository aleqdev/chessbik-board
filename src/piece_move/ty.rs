#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PieceMoveTy {
    Slide,
    Take,
    Castle,
}
