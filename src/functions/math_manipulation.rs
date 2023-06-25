pub fn inv_sqrt(float: f32) -> f32 {
    let halv: f32 = 0.5 * float;
    let f32_from_i32: f32 = f32::from_bits(
        0x5f3759df - (float.to_bits() >> 1)
    );

    f32_from_i32 * 1.5 - halv * f32_from_i32.powi(3)
}

pub fn rot_left(num: u32, shift: u32) -> u32 {
    num.wrapping_shl(shift) | num.wrapping_shr(32_u32.wrapping_sub(shift))
}

pub fn rot_right(num: u32, shift: u32) -> u32 {
    num.wrapping_shr(shift) | num.wrapping_shl(32_u32.wrapping_sub(shift))
}

pub fn factorial(n: u32) -> f64 {
    (2..=n).fold(1.0, |acc: f64, x: u32| acc * x as f64)
}

pub fn power(x: f64, n: u32) -> f64 {
    (0..n).fold(1.0, |acc: f64, _| acc * x)
}

// sin: sum of { (-1)^n * x^(2n+1) / (2n+1)! }: n -> 0..=inf
// cos: sum of { (-1)^n * x^ 2n    / (2n  )! }: n -> 0..=inf

pub fn sin(x: f64) -> f64 {
    const TERMS: u32 = 10;
    let mut result: f64 = 0.0;
    let mut sign: f64 = 1.0;

    for i in 0..TERMS {
        let term: f64 = power(x, 2 * i + 1) / factorial(2 * i + 1);
        result += sign * term;
        sign *= -1.0;
    }

    result
}

pub fn cos(x: f64) -> f64 {
    const  TERMS: u32 = 10;
    let mut result: f64 = 0.0;
    let mut sign: f64 = 1.0;

    for i in 0..TERMS {
        let term: f64 = power(x, 2 * i) / factorial(2 * i);
        result += sign * term;
        sign *= -1.0
    }

    result
}

pub fn sin_cos(x: f64) -> (f64, f64) {
    const TERMS: u32 = 10;

    let mut sin_result: f64 = 0.0;
    let mut cos_result: f64 = 0.0;
    let mut sign: f64 = 1.0;

    for i in 0..TERMS {
        let cos_term: f64 = power(x, 2 * i) / factorial(2 * i);
        let sin_term: f64 = power(x, 2 * i + 1) / factorial(2 * i + 1);

        cos_result += sign * cos_term;
        sin_result += sign * sin_term;

        sign *= -1.0;
    }

    (sin_result, cos_result)
}

fn build_f64(mantissa: u64, exponent: i16, sign: i8) -> f64 {
    let mut value = mantissa as f64;
    value *= 2.0_f64.powi(exponent as i32);
    if sign.signum() == -1 {
        value *= -1.;
    }
    value
}