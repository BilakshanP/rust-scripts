pub mod macros;

module!( pub base, hash, enc_dec, functions, mathematica, prelude, data_structures, algorithms );

use std::mem::forget;

use mathematica::matrices::{Matrix as M, old::OldMatrix};

fn main() {
    let x = 9;

    forget(x)
}