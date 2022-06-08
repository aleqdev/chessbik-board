use crate::BoardNew;

use super::*;

impl<T: BoardNew> Default for Board<T> {
    fn default() -> Self {
        T::board_new()
    }
}
