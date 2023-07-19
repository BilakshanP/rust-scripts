#![allow(unused)]

pub mod macros;


module!( pub base, hash, enc_dec, functions, prelude, data_structures, algorithms, mathematica );

use mathematica::matrices::{Matrix as M, old::OldMatrix as OM};
use mathematica::vectors::{Vector3D as V3, Vector2P as V2};
use mathematica::angles::Angle;


fn main() {
    println!("{}", V3::new(3., 4., 0.).magnitude_sq());
}

use rand::{Rng, rngs::ThreadRng};

pub fn gen_random<T: std::cmp::PartialOrd + rand::distributions::uniform::SampleUniform>(start: T, end: T, thread: &mut ThreadRng) -> T {
    thread.gen_range(start..=end)
}