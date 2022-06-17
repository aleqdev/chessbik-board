use crate::{Eval, GetEval};

use super::*;

impl GetEval for PieceTy {
    fn get_eval(&self) -> Eval {
        match self {
            PieceTy::PAWN => Eval(2.),
            PieceTy::ROOK => Eval(5.),
            PieceTy::KNIGHT => Eval(3.),
            PieceTy::BISHOP => Eval(3.),
            PieceTy::QUEEN => Eval(9.),
            PieceTy::KING => Eval(999.),
            PieceTy::MAGE => Eval(3.),
        }
    }
}
