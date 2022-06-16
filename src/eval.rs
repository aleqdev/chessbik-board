chessbik_derive_wrapper::derive_wrapper!(
    #[derive(
        derive_more::Mul,
        derive_more::Add,
        derive_more::Sub,
        derive_more::Div,
        Debug,
        Clone,
        Copy,
        PartialEq,
        PartialOrd,
    )]
    pub struct Eval(pub f32);
);
