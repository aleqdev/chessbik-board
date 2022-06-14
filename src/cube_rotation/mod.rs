use enum_primitive_derive::Primitive;

pub mod impls;
pub use impls::*;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize, Primitive)]
pub enum CubeRotation {
    U = 0,
    D = 1,
    R = 2,
    L = 3,
    F = 4,
    B = 5,
    U2 = 6,
    D2 = 7,
    R2 = 8,
    L2 = 9,
    F2 = 10,
    B2 = 11,
    U_P = 12,
    D_P = 13,
    R_P = 14,
    L_P = 15,
    F_P = 16,
    B_P = 17,
    //U_P2,
    //D_P2,
    //R_P2,
    //L_P2,
    //F_P2,
    //B_P2,
    M = 18,
    E = 19,
    S = 20,
    M2 = 21,
    E2 = 22,
    S2 = 23,
    M_P = 24,
    E_P = 25,
    S_P = 26
}