mod functions;

use functions::math_manipulation::{factorial, power, sin, cos};

fn sine_cosine(x: f64) -> (f64, f64) {
    const TERMS: u32 = 10;
    let mut sin_result: f64 = 0.0;
    let mut cos_result: f64 = 0.0;
    let mut sin_sign: f64 = 1.0;
    let mut cos_sign: f64 = 1.0;

    for i in 0..TERMS {
        let sin_term: f64 = power(x, 2 * i + 1) / factorial(2 * i + 1);
        let cos_term: f64 = power(x, 2 * i) / factorial(2 * i);

        sin_result += sin_sign * sin_term;
        cos_result += cos_sign * cos_term;

        sin_sign *= -1.0;
        cos_sign *= -1.0;
    }

    (sin_result, cos_result)
}

// sin: sum of { (-1)^n * x^(2n+1) / (2n+1)! }: n -> 0..=inf
// cos: sum of { (-1)^n * x^ 2n    / (2n  )! }: n -> 0..=inf
// create a new function named sin_cos which returns both sin and cos simultaneously taking above two function's optimizations into account

fn main() {
    let angle: f64 = 0.5; // Angle in radians
    let sc = sine_cosine(angle);
    println!("sin: {}\n     {}", angle.sin(), sc.0);
    println!("cos: {}\n     {}", angle.cos(), sc.1)
}
