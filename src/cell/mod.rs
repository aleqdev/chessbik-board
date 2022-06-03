pub mod impls;
pub use impls::*;

use crate::Piece;

chessbik_commons::derive_wrapper!(
    #[derive(Clone, Copy)]
    pub struct Cell {
        pub value: Option<Piece>,
    }
);