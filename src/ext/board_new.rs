use crate::{Board, Piece};

pub trait BoardNew {
    fn board_new<T: AsRef<Piece>>() -> Board<T>;
}
