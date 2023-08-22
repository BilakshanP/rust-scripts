type Real = f64;
type Imaginary = f64;

use num::Zero;

/// Quake III inv sqrt algorothm with 0 newtonian iterations.
/// Use [inv_sqrt_accuracy] for higher accuacy.
pub fn inv_sqrt(float: f64) -> f64 {
    let f32_from_i32: f64 = f64::from_bits(
        0x5fe6eb50c7b537a9 - (float.to_bits() >> 1)
    );

    f32_from_i32
}

/// Accuracy is clamped between 0 and 6, for 0 use [inv_sqrt] instead.
pub fn inv_sqrt_accuracy(float: f64, accuracy: u8) -> f64 {
    let halv: f64 = 0.5 * float;
    let mut f32_from_i32: f64 = f64::from_bits(
        0x5fe6eb50c7b537a9 - (float.to_bits() >> 1)
    );

    for _ in 0..accuracy.clamp(0, 6) {
        f32_from_i32 = f32_from_i32 * 1.5 - halv * f32_from_i32.powi(3);
    }

    f32_from_i32
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

pub fn build_f64(mantissa: u64, exponent: i16, sign: i8) -> f64 {
    let mut value = mantissa as f64;
    value *= 2.0_f64.powi(exponent as i32);
    if sign.signum() == -1 {
        value *= -1.;
    }
    value
}

pub fn solve_quadratic(a: f64, b: f64, c: f64) -> ((Real, Imaginary), (Real, Imaginary)) {
    let div_2a: f64 = 0.5 / a;
    let pre_det = b.mul_add(b, -4.0 * a * c);

    if pre_det == 0.0 {
        let root: (f64, f64) = (-b * div_2a, 0.0);
        return (root, root);
    }

    let det: f64 = pre_det.abs().sqrt();

    match pre_det.partial_cmp(&0.0) {
        Some(std::cmp::Ordering::Greater) => (
            ((-b + det) * div_2a, 0.0),
            ((-b - det) * div_2a, 0.0),
        ),
        _ => {
            let real: f64 = -b * div_2a;
            let imaginary: f64 = det * div_2a;
            ((real, imaginary), (real, -imaginary))
        }
    }
}

pub fn split<T: Copy + From<u8> + std::cmp::PartialOrd + std::ops::DivAssign<T> + std::ops::Rem<Output = T>>(num: T) -> Vec<T> {
    let mut n: T = num;
    let mut digits: Vec<T> = Vec::new();

    let ten: T = T::from(10);

    while n > T::from(0) {
        digits.push(n % ten);
        n /= ten;
    }

    digits
}

pub fn big_split(num: num::BigInt) -> Vec<num::BigInt> {
    let mut n: num::BigInt = num;
    let mut digits: Vec<num::BigInt> = Vec::new();

    let ten: num::BigInt = num::BigInt::from(10);

    while n > num::BigInt::zero() {
        digits.push(n.clone() % ten.clone());
        n /= ten.clone();
    }

    digits
}