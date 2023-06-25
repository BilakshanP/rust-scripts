//#![allow(dead_code, unused)]

pub mod hash;
pub mod functions;

use hash::sha::sha2::_256::{sha256, digest};

fn main() {
    let data = b"Hello, World!";

    let hash = sha256(data);
    let dig = digest(&hash);
    println!("Correct: {:?}", "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f7a4b28688a362182986f");

    println!("To Test: {:?}", dig)
}