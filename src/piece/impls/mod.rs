use crate::GetPiece;

pub use super::*;

pub mod get_eval;
pub use get_eval::*;

pub mod display;
pub use display::*;

pub mod get_available_moves;
pub use get_available_moves::*;

pub mod get_piece;
pub use get_piece::*;

impl Piece {
    pub const WHITE_PAWN: Self = Self {
        ty: PieceTy::PAWN,
        color: PieceColor::WHITE,
    };

    pub const BLACK_PAWN: Self = Self {
        ty: PieceTy::PAWN,
        color: PieceColor::BLACK,
    };

    pub const WHITE_KNIGHT: Self = Self {
        ty: PieceTy::KNIGHT,
        color: PieceColor::WHITE,
    };

    pub const BLACK_KNIGHT: Self = Self {
        ty: PieceTy::KNIGHT,
        color: PieceColor::BLACK,
    };

    pub const WHITE_BISHOP: Self = Self {
        ty: PieceTy::BISHOP,
        color: PieceColor::WHITE,
    };

    pub const BLACK_BISHOP: Self = Self {
        ty: PieceTy::BISHOP,
        color: PieceColor::BLACK,
    };

    pub const WHITE_ROOK: Self = Self {
        ty: PieceTy::ROOK,
        color: PieceColor::WHITE,
    };

    pub const BLACK_ROOK: Self = Self {
        ty: PieceTy::ROOK,
        color: PieceColor::BLACK,
    };

    pub const WHITE_QUEEN: Self = Self {
        ty: PieceTy::QUEEN,
        color: PieceColor::WHITE,
    };

    pub const BLACK_QUEEN: Self = Self {
        ty: PieceTy::QUEEN,
        color: PieceColor::BLACK,
    };

    pub const WHITE_KING: Self = Self {
        ty: PieceTy::KING,
        color: PieceColor::WHITE,
    };

    pub const BLACK_KING: Self = Self {
        ty: PieceTy::KING,
        color: PieceColor::BLACK,
    };

    pub const WHITE_MAGE: Self = Self {
        ty: PieceTy::MAGE,
        color: PieceColor::WHITE,
    };

    pub const BLACK_MAGE: Self = Self {
        ty: PieceTy::MAGE,
        color: PieceColor::BLACK,
    };
}

/*impl<U> GetPiece for U
where
    U: AsRef<Piece>
{
    fn get_piece(&self) -> Option<&crate::Piece> {
        self.as_ref().get_piece()
    }
}

impl<T, U> GetAvailableMoves<T> for U
where
    U: AsRef<Piece>,
    T: AsRef<Piece>
{
    fn get_available_moves(
        &self,
        pos: impl Into<crate::PiecePosition>,
        board: &crate::Board<impl AsRef<Piece>>,
    ) -> Vec<crate::PieceMove> {
        self.as_ref().get_available_moves(pos, board)
    }
}*/