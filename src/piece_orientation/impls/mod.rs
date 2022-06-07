use std::f32::consts::PI;

pub use super::*;

pub mod default;
pub use default::*;

impl PieceOrientation {
    pub fn to_radians(&self) -> f32 {
        match self {
            PieceOrientation::FORWARD => 0.,
            PieceOrientation::RIGHT => PI / 2.,
            PieceOrientation::BACK => PI,
            PieceOrientation::LEFT => -PI / 2.,
        }
    }
}
