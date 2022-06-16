use num_traits::FromPrimitive;

use crate::{
    cube_rotations_field, shape_geodesic_field, Board, BoardTransform, CubeRotation,
    GetAvailableMoves, PieceMove, PiecePosition,
};

use super::*;

impl<T> GetAvailableMoves<T> for Piece
where
    T: GetPiece + GetAvailableMoves<T> + Copy + serde::Serialize + BoardTransform,
{
    fn get_available_moves(
        &self,
        pos: impl Into<PiecePosition>,
        board: &Board<T>,
    ) -> Vec<PieceMove> {
        let pos = pos.into();

        let mut moves = match self.ty {
            PieceTy::PAWN => {
                let mut v = vec![];

                v.extend(
                    shape_geodesic_field::geodesic_calculator(pos, self.color, ..1, ..0, board)
                        .iter()
                        .filter(|m| match m {
                            PieceMove::Slide(..) => true,
                            _ => false,
                        }),
                );

                v.extend(
                    shape_geodesic_field::geodesic_calculator(pos, self.color, ..0, ..1, board)
                        .iter()
                        .filter(|m| match m {
                            PieceMove::Take(..) => true,
                            _ => false,
                        }),
                );

                v
            }
            PieceTy::ROOK => {
                shape_geodesic_field::geodesic_calculator(pos, self.color, .., ..0, board)
            }
            PieceTy::KNIGHT => shape_geodesic_field::KNIGHT_FIELD[*pos]
                .iter()
                .filter_map(|&i| match board.at(i).get_piece() {
                    Some(at) => {
                        if at.color != self.color {
                            Some(PieceMove::take(i))
                        } else {
                            None
                        }
                    }
                    None => Some(PieceMove::slide(i)),
                })
                .collect(),
            PieceTy::BISHOP => {
                shape_geodesic_field::geodesic_calculator(pos, self.color, ..0, .., board)
            }
            PieceTy::QUEEN => {
                shape_geodesic_field::geodesic_calculator(pos, self.color, .., .., board)
            }
            PieceTy::KING => {
                shape_geodesic_field::geodesic_calculator(pos, self.color, ..1, ..1, board)
            }
            PieceTy::MAGE => {
                let mut v =
                    shape_geodesic_field::geodesic_calculator(pos, self.color, ..1, ..1, board);

                for i in 0..27 {
                    let pair = cube_rotations_field::FIELD[i];
                    if pair[0].contains(&pos) {
                        v.push(PieceMove::Rotation(CubeRotation::from_usize(i).unwrap()));
                    }
                }

                v
            }
        };

        moves.retain(|m| {
            let mut new_board = board.clone();
            new_board.apply_move_unchecked(*m, Some(pos));
            new_board.validate(self.color)
        });

        moves
    }
}
