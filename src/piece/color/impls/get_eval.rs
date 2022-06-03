use crate::{GetEval, Eval};

use super::*;

impl GetEval for PieceColor {
    fn get_eval(&self) -> Eval {
        match self {
            PieceColor::WHITE => Eval(1.),
            PieceColor::BLACK => Eval(-1.),
        }
    }
}