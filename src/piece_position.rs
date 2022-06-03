chessbik_commons::derive_wrapper!(
    #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
    pub struct PiecePosition(pub usize);
);
