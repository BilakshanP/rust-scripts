// check comments at last

/// set_zero(&mut var)
pub fn set_zero(x: &mut i32) {
    *x = 0;
}

#[allow(clippy::unused_unit)]
pub fn increment<NumberType>(num: &mut NumberType) -> ()
where NumberType: std::ops::Add<Output = NumberType>
    + Copy
    + From<u8>,
{
    *num = *num + NumberType::from(1_u8)
}

// macro_rules! bundle_traits {
//     ( $trait_name: expr, $( $trait:tt )+ ) => {
//         trait $trait_name: $( $trait )+ {}
// 
//         impl<T> $trait_name for T where T: $( $trait )+ {}
//     };
//     
//     ( $trait_name: expr, $( $trait:ident $(: $generic:tt )? ),+ ) => {
//         trait $trait_name: $( $trait $(: $generic)? )+ {}
// 
//         impl<T> $trait_name for T where T: $( $trait $(: $generic)? )+ {}
//     };
// }

/*
    use std::ops::Add;

use functions::{
    base64 as b64,
    compression::{
        compress_data,
        decompress_data
    },
    conversions as cnv
};

trait Numeric: Copy
    + Default
    + PartialEq
    + PartialOrd
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self> {
        /* ... */
}

// Implement the trait for built-in numeric types
impl<T> Numeric for T
where T: Copy
    + Default
    + PartialEq
    + PartialOrd
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self> {
        /* ... */
}

// Define the Matrix struct with the Numeric trait bound
struct Matrix<T: Numeric> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Numeric> Matrix<T> {}

trait Inc {
    fn increment(&mut self);
}

impl <NumberType> Inc for NumberType
where NumberType: std::ops::Add<Output = NumberType>
    + Copy
    + From<u8>
{
    fn increment(&mut self) {
        *self = *self + NumberType::from(1_u8)
    }
}
 */