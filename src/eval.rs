chessbik_commons::derive_wrapper!(
    #[derive(derive_more::Mul, derive_more::Add, derive_more::Sub, derive_more::Div)]
    pub struct Eval(pub f32);
);
