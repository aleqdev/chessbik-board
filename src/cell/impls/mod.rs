use crate::{Piece, PieceColor, PieceTy};

pub use super::*;

pub mod display;
pub use display::*;

impl Cell {
    pub const fn some(piece: Piece) -> Cell {
        Cell { value: Some(piece) }
    }

    pub const fn none() -> Cell {
        Cell { value: None }
    }

    pub const WPAWN: Cell = Self::some(Piece {
        ty: PieceTy::PAWN,
        color: PieceColor::WHITE,
    });

    pub const BPAWN: Cell = Self::some(Piece {
        ty: PieceTy::PAWN,
        color: PieceColor::BLACK,
    });

    pub const WKNIGHT: Cell = Self::some(Piece {
        ty: PieceTy::KNIGHT,
        color: PieceColor::WHITE,
    });

    pub const BKNIGHT: Cell = Self::some(Piece {
        ty: PieceTy::KNIGHT,
        color: PieceColor::BLACK,
    });

    pub const WBISHOP: Cell = Self::some(Piece {
        ty: PieceTy::BISHOP,
        color: PieceColor::WHITE,
    });

    pub const BBISHOP: Cell = Self::some(Piece {
        ty: PieceTy::BISHOP,
        color: PieceColor::BLACK,
    });

    pub const WROOK: Cell = Self::some(Piece {
        ty: PieceTy::ROOK,
        color: PieceColor::WHITE,
    });

    pub const BROOK: Cell = Self::some(Piece {
        ty: PieceTy::ROOK,
        color: PieceColor::BLACK,
    });

    pub const WQUEEN: Cell = Self::some(Piece {
        ty: PieceTy::QUEEN,
        color: PieceColor::WHITE,
    });

    pub const BQUEEN: Cell = Self::some(Piece {
        ty: PieceTy::QUEEN,
        color: PieceColor::BLACK,
    });

    pub const WKING: Cell = Self::some(Piece {
        ty: PieceTy::KING,
        color: PieceColor::WHITE,
    });

    pub const BKING: Cell = Self::some(Piece {
        ty: PieceTy::KING,
        color: PieceColor::BLACK,
    });

    pub const WMAGE: Cell = Self::some(Piece {
        ty: PieceTy::MAGE,
        color: PieceColor::WHITE,
    });

    pub const BMAGE: Cell = Self::some(Piece {
        ty: PieceTy::MAGE,
        color: PieceColor::BLACK,
    });

    pub const NONE: Cell = Self::none();
}
