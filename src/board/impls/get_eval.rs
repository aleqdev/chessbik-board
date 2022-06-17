use crate::{Eval, GetEval};

use super::*;

impl<T> GetEval for Board<T> 
where T: AsRef<Piece> + serde::Serialize + GetPiece + GetAvailableMoves<T> + Copy + BoardTransform
{
    fn get_eval(&self) -> Eval {
        let mut e = Eval(0.);

        for i in 0..54 {
            match self.at(i).get_piece() {
                Some(p) => e.0 += p.get_eval().0,
                None => {}
            }
        }

        e
    }
}
