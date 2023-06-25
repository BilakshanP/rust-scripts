#[macro_export]
macro_rules! _test {
    ($e:expr) => {
        println!("{}={}", stringify!($e), $e)
    };
}

#[macro_export]
macro_rules! bundle_traits {
    ($trait_name:ident, $($traits:path),+) => {
        trait $trait_name: $($traits +)+ {}
        impl<T: $($traits +)+> $trait_name for T {}
    };
}