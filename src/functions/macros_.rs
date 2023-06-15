macro_rules! _test {
    ($e:expr) => {
        println!("{}={}", stringify!($e), $e)
    };
}

// macro_rules! bundle_traits {
//     ( $trait_name: expr, $( $trait:tt )+ ) => {
//         trait $trait_name: $( $trait )+ {}
// 
//         impl<T> $trait_name for T where T: $( $trait )+ {}
//     };
//     
//     ( $trait_name: expr, $( $trait:ident $(: $generic:tt )? ),+ ) => {
//         trait $trait_name: $( $trait $(: $generic)? )+ {}
// 
//         impl<T> $trait_name for T where T: $( $trait $(: $generic)? )+ {}
//     };
// }