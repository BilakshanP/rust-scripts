#![allow(unused, unreachable_code)]

use std::collections::HashSet;
use std::hash::{DefaultHasher, Hasher};
use std::io::Read;
use std::time;

use aes::cipher::consts::False;
use ml_kem::array::sizes;
use num::Integer;

mod macros;

module!(
    base,
    hash,
    enc_dec,
    functions,
    data_structures,
    algorithms,
    mathematica
);

struct Solution;

fn spiral(v: Vec<Vec<i32>>) -> Vec<i32> {
    let out_vec = vec![-1; v.len()];

    let x = v.first().unwrap_or(&vec![]).len();

    out_vec
}

fn main() {
    let a = 12342;
    let b = 126;

    let gcd = a.gcd(&b);

    println!("GCD: {}", gcd);

    assert_eq!(gcd, gcd_modulous(a, b));
    assert_eq!(gcd, gcd_subtract(a, b))
}

fn gcd_modulous(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        (a, b) = (b, a % b)
    }
    a
}

fn gcd_subtract(mut a: i32, mut b: i32) -> i32 {
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a
        }
    }

    a
}

pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();

    for ch in s.chars() {
        match ch {
            '(' => stack.push(')'),
            '[' => stack.push(')'),
            '{' => stack.push('}'),
            ch => {
                if Some(ch) != stack.pop() {
                    return false;
                }
            }
        }
    }

    stack.is_empty()
}
