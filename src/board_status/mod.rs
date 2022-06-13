#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum BoardStatus {
    WhitesMove,
    BlacksMove,
    Mate
}