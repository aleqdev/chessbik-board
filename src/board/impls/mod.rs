use crate::{GetAvailableMoves, GetPiece, PieceMove, PiecePosition};

pub use super::*;

pub mod get_eval;
pub use get_eval::*;

pub mod print_debug;
pub use print_debug::*;

pub mod default;
pub use default::*;

impl<T: GetPiece + GetAvailableMoves<T> + Copy> Board<T> {
    pub fn at<'a>(&'a self, pos: impl Into<PiecePosition>) -> &'a T {
        &self.cells[*pos.into()]
    }

    pub fn translate(&mut self, from: impl Into<PiecePosition>, to: impl Into<PiecePosition>) {
        self.cells[*to.into()] = self.cells[*from.into()];
    }

    pub fn get_available_moves(&self, pos: impl Into<PiecePosition>) -> Vec<PieceMove> {
        let pos = pos.into();
        let cell = &self.cells[*pos];
        cell.get_available_moves(pos, self)
    }
}
