#[derive(Debug)]
pub enum LogarithimError {
    DifferentBase(f64, f64),
    InvalidArgument(f64),
    InvalidBase(f64),
    NumberNegativeExpopent(f64),

    UnknownError
}

#[derive(Debug)]
pub enum MatrixError {
    InvalidMatrix,
    InvalidMatrixArray,
    MismatchedSize {
        expected: usize,
        provided: usize
    },
    NonSquareMatrix,
    SingularMatrix,
    NullMatrix,
    NullDeterminant,
    UnEvenRows(usize),

    UnknownError
}

pub enum VectorError {

    UnknownError
}