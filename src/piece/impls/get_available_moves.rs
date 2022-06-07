use crate::{shape_geodesic_field, Board, GetAvailableMoves, GetPiece, PieceMove, PiecePosition};

use super::*;

impl GetAvailableMoves<Piece> for Piece {
    fn get_available_moves(
        &self,
        pos: impl Into<PiecePosition>,
        board: &Board<Piece>,
    ) -> Vec<PieceMove> {
        let pos = pos.into();

        match self.ty {
            PieceTy::PAWN => vec![],
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
                shape_geodesic_field::geodesic_calculator(pos, self.color, ..1, ..1, board)
            }
        }
    }
}
