use crate::Board;

pub trait BoardNew {
    fn board_new<T>() -> Board<T>;
}
