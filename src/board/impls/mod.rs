use std::cell::RefCell;

use crate::{shape_geodesic_field, GetAvailableMoves, GetPiece, PieceMove, PiecePosition, PieceTy, BoardTransform, cube_rotations_field, PieceColor};

pub use super::*;

pub mod get_eval;
pub use get_eval::*;

pub mod print_debug;
pub use print_debug::*;

pub mod default;
pub use default::*;

impl<T: GetPiece + GetAvailableMoves<T> + Copy + serde::Serialize + BoardTransform> Board<T> {
    pub fn at<'a>(&'a self, pos: impl Into<PiecePosition>) -> &'a T {
        &self.cells[*pos.into()]
    }

    pub fn at_mut<'a>(&'a mut self, pos: impl Into<PiecePosition>) -> &'a mut T {
        &mut self.cells[*pos.into()]
    }

    pub fn get_available_moves(&self, pos: impl Into<PiecePosition>) -> Vec<PieceMove> {
        let pos = pos.into();
        let cell = &self.cells[*pos];
        cell.get_available_moves(pos, self)
    }

    pub fn apply_move_unchecked(&mut self, m: PieceMove, from: Option<PiecePosition>) {
        match m {
            PieceMove::Slide(m) |
            PieceMove::Take(m) => {
                let from = from.unwrap();
                let at = *self.at(from);
                self.at_mut(m).by_slide(&at);
                self.at_mut(from).remove();
            }
            PieceMove::Rotation(rot) => {
                let pairs = cube_rotations_field::get_positions(rot);

                let mut new_board = self.clone();

                for (&from, &to) in pairs[0].iter().zip(pairs[1].iter()) {
                    new_board.at_mut(to).by_rotation(self.at(from));
                }

                *self = new_board;
            }
        }

        match self.status {
            BoardStatus::WhitesMove => self.status = BoardStatus::BlacksMove,
            BoardStatus::BlacksMove => self.status = BoardStatus::WhitesMove,
            BoardStatus::Mate => {},
        }
    }

    pub fn validate(&self, color_of_king: PieceColor) -> bool {
        thread_local! {
            static LAST_WHITE_KING_POS: RefCell<PiecePosition> = RefCell::new(PiecePosition(0));
            static LAST_BLACK_KING_POS: RefCell<PiecePosition> = RefCell::new(PiecePosition(0));
        }

        match color_of_king {
            PieceColor::WHITE => &LAST_WHITE_KING_POS,
            PieceColor::BLACK => &LAST_BLACK_KING_POS
        }.with(|last_pos| {
            let mut last_pos = last_pos.borrow_mut();

            if self
                .at(*last_pos)
                .get_piece()
                .map_or(true, |p| p.ty != PieceTy::KING || p.color != color_of_king)
            {
                for i in 0..54 {
                    if self
                        .at(i)
                        .get_piece()
                        .map_or(false, |p| p.ty == PieceTy::KING && p.color == color_of_king)
                    {
                        *last_pos = i.into();
                        break;
                    }
                }
            }

            println!("color: {:?}. pos: {:?}", color_of_king, last_pos);

            for m in shape_geodesic_field::geodesic_calculator(*last_pos, color_of_king, .., ..0, self) {
                match m {
                    PieceMove::Take(pos) => {
                        if [PieceTy::QUEEN, PieceTy::ROOK]
                            .contains(&self.at(pos).get_piece().unwrap().ty)
                        {
                            return false;
                        }
                    }
                    _ => {}
                }
            }

            for m in shape_geodesic_field::geodesic_calculator(*last_pos, color_of_king, ..0, .., self) {
                match m {
                    PieceMove::Take(pos) => {
                        if [PieceTy::QUEEN, PieceTy::BISHOP]
                            .contains(&self.at(pos).get_piece().unwrap().ty)
                        {
                            return false;
                        }
                    }
                    _ => {}
                }
            }

            for pos in shape_geodesic_field::KNIGHT_FIELD[**last_pos] {
                match self.at(*pos).get_piece() {
                    Some(piece) => {
                        if piece.color != color_of_king && piece.ty == PieceTy::KNIGHT {
                            return false;
                        }
                    }
                    None => {}
                }
            }

            for m in shape_geodesic_field::geodesic_calculator(*last_pos, color_of_king, ..1, ..1, self) {
                match m {
                    PieceMove::Take(pos) => {
                        if [PieceTy::KING, PieceTy::MAGE]
                            .contains(&self.at(pos).get_piece().unwrap().ty)
                        {
                            return false;
                        }
                    }
                    _ => {}
                }
            }

            for m in shape_geodesic_field::geodesic_calculator(*last_pos, color_of_king, ..1, ..0, self) {
                match m {
                    PieceMove::Take(pos) => {
                        if self.at(pos).get_piece().unwrap().ty == PieceTy::PAWN {
                            return false;
                        }
                    }
                    _ => {}
                }
            }

            true
        })
    }
}
