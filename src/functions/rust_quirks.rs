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