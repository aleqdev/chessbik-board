use crate::Board;

pub trait BoardNew {
    fn board_new() -> Board<Self> where Self: Sized + serde::Serialize;
}
