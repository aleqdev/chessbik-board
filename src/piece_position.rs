chessbik_derive_wrapper::derive_wrapper!(
    #[derive(
        Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
    )]
    pub struct PiecePosition(pub usize);
);
