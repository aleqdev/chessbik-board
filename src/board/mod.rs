use crate::Cell;

pub mod impls;

pub struct Board {
    pub cells: [Cell; 9 * 6],
}