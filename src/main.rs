#![allow(unused)]

pub mod macros;


module!( pub base, hash, enc_dec, functions, prelude, data_structures, algorithms, mathematica );

use mathematica::matrices::{Matrix as M, old::OldMatrix as OM};
use mathematica::vectors::{Vector3D as V3, Vector2P as V2};
use mathematica::angles::Angle;


fn main() {
    let vec_1 = V3::new(3., 1., 4.);
    let vec_2 = V3::new(1., -1., 1.);
    let vec_3 = V3::new(2., 3., 1.);

    println!("{}", V2::new(vec_1, vec_2).area_parallelogram_self());
}

use rand::{Rng, rngs::ThreadRng};

pub fn gen_random<T: std::cmp::PartialOrd + rand::distributions::uniform::SampleUniform>(start: T, end: T, thread: &mut ThreadRng) -> T {
    thread.gen_range(start..=end)
}