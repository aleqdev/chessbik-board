use crate::{Eval, GetEval};

use super::*;

impl<T: AsRef<Piece>> GetEval for Board<T> {
    fn get_eval(&self) -> Eval {
        todo!()
    }
}