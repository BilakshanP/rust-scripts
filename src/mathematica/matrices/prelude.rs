#[derive(Debug, Clone)]
pub enum Info<T> {
    Value(Option<T>),
    NotCalculated
}

pub type Mat = Vec<Vec<f64>>;

pub use super::Matrix;
pub use super::super::errors::MatrixError;

pub use Info::{Value, NotCalculated as NC};
pub use MatrixError::*;

macro_rules! bundle {
    ( $trait_name:ident, $($traits:path),+ ) => {
        pub trait $trait_name: $($traits +)+ {}
        impl<T: $($traits +)+> $trait_name for T {}
    };
}

bundle!(Number, num::Num, Clone, num::pow::Pow<f64, Output = Self>, std::fmt::Debug); //, CheckedMul);