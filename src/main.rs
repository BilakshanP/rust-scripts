mod functions;

macro_rules! _test {
    ($e:expr) => {
        println!("{}={}", stringify!($e), $e)
    };
}

fn factorial(n: u32) -> f64 {
    /* if n <= 1 {
        1.0
    } else {
        (2..=n).fold(1.0, |acc: f64, x: u32| acc * x as f64)
    } */

    (2..=n).fold(1.0, |acc: f64, x: u32| acc * x as f64)
}

fn power(x: f64, n: u32) -> f64 {
    (0..n).fold(1.0, |acc: f64, _| acc * x)
}

fn sin(x: f64) -> f64 {
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

fn main() {
    let angle: f64 = 0.5; // Angle in radians
    println!("{}\n{}", angle.sin(), sin(angle));
}
