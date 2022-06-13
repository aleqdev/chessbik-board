use crate::BoardNew;

use super::*;

impl<T: BoardNew + serde::Serialize> Default for Board<T> {
    fn default() -> Self {
        T::board_new()
    }
}
