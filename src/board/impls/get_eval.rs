use crate::{Eval, GetEval};

use super::*;

impl<T> GetEval for Board<T> 
where T: AsRef<Piece> + serde::Serialize + GetPiece + GetAvailableMoves<T> + Copy + BoardTransform + GetEval
{
    fn get_eval(&self) -> Eval {
        let mut e = Eval(0.);

        for i in 0..54 {
            e.0 += self.at(i).get_eval().0;
        }

        e
    }
}
