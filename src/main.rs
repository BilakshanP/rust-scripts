#![allow(unused, unreachable_code)]

pub mod macros;

module!( pub base, hash, enc_dec, functions, data_structures, algorithms, mathematica );

fn range_sum_0(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn range_sum_mn(m: i32, n: i32) -> i32 {
    m - range_sum_0(m) + range_sum_0(n)
}

fn main() {
    let m = 88;
    let n = 99;
}
