pub mod macros;

r#mod!( pub base, hash, enc_dec, functions, mathematica, prelude, data_structures, algorithms );

// Selection, Insertion, Bubble, Quick, Shell, STD, Radix <- Bucket extension
//                          n*n            nlog(n)      n



fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1: {}", *r1);
        println!("r1: {}", *r2);
    }

    let num_1 = 99;

    mutate!( num_1, mut_num_1, i32 );

    unsafe { *mut_num_1 = 90 }

    println!("{}", num_1);

    union Lol {
        i: i32,
        j: i32
    }
}