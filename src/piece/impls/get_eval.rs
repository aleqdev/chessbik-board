use crate::{GetEval, Eval};

use super::*;

impl GetEval for Piece {
    fn get_eval(&self) -> Eval {
        self.ty.get_eval() * *self.color.get_eval()
    }
}