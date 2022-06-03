use crate::Eval;

pub trait GetEval {
    fn get_eval(&self) -> Eval;
}