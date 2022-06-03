use crate::PiecePosition;

pub use super::*;

pub mod default;
pub use default::*;

pub mod get_eval;
pub use get_eval::*;

pub mod board_new;
pub use board_new::*;

pub mod print_debug;
pub use print_debug::*;

pub mod get_available_moves;
pub use get_available_moves::*;

impl Board {
    pub fn at<'a>(&'a self, fref: impl Into<PiecePosition>) -> &'a Cell {
        &self.cells[*fref.into()]
    }

    pub fn translate(&mut self, from: impl Into<PiecePosition>, to: impl Into<PiecePosition>) {
        self.cells[*to.into()] = self.cells[*from.into()];
    }
}
