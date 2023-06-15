pub fn inv_sqrt(float: f32) -> f32 {
    let halv: f32 = 0.5 * float;
    let f32_from_i32: f32 = f32::from_bits(
        1597463007 - (float.to_bits() >> 1)
    );

    f32_from_i32 * 1.5 - halv * f32_from_i32.powi(3)
}

pub fn factorial(n: u32) -> f64 {
    /* if n <= 1 {
        1.0
    } else {
        (2..=n).fold(1.0, |acc: f64, x: u32| acc * x as f64)
    } */

    (2..=n).fold(1.0, |acc: f64, x: u32| acc * x as f64)
}

pub fn power(x: f64, n: u32) -> f64 {
    (0..n).fold(1.0, |acc: f64, _| acc * x)
}

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