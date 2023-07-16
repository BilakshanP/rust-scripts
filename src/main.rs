pub mod macros;

module!( pub base, hash, enc_dec, functions, mathematica, prelude, data_structures, algorithms );


use mathematica::matrices::{Matrix as M, old::OldMatrix as OM};

fn main() {
    let m1 = vec![
        vec![1., 0., 2.],
        vec![0., 1., 0.],
        vec![0., 0., 1.]
    ];

    let m2 = vec![
        vec![3., 7., 2.],
        vec![7., 9., 2.],
        vec![1., 4., 7.]
    ];

    let m1_o = OM::new_from_vec_unsized(m1.clone());
    let m2_o = OM::new_from_vec_unsized(m2.clone());

    let m1_n = M::new_from_vec(m1).unwrap();
    let m2_n = M::new_from_vec(m2).unwrap();

    println!("{}", m2_o.inverse().unwrap());
    println!("{}", m2_n.inverse().unwrap());
}