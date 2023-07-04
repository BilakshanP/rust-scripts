#[macro_export]
macro_rules! _test {
    ( $e:expr ) => {
        println!("{}={}", stringify!($e), $e)
    };
}

#[macro_export]
macro_rules! bundle_traits {
    ( $trait_name:ident, $($traits:path),+ ) => {
        trait $trait_name: $($traits +)+ {}
        impl<T: $($traits +)+> $trait_name for T {}
    };
}

/// If capacity is defined before the types it changes to an immutable, in all other cases it's mutable
#[macro_export]
macro_rules! hash_map {
    ( $hash_name:ident, $(($key:expr, $value:expr)),* ) => {
        let $hash_name: std::collections::HashMap<_, _> = [ $( ($key, $value) )* ]
                                                                .iter()
                                                                .cloned()
                                                                .collect();
    };

    ( mut $hash_name:ident, $(($key:expr, $value:expr)),* ) => {
        let mut $hash_name: std::collections::HashMap<_, _> = [ $( ($key, $value) )* ]
                                                                .iter()
                                                                .cloned()
                                                                .collect();
    };

    ( $hash_name:ident, $from_type:ty, $to_type:ty, $(($key:expr, $value:expr)),* ) => {
        let $hash_name: std::collections::HashMap<from_type, to_type> = [ $( ($key, $value) )* ]
                                                                                .iter()
                                                                                .cloned()
                                                                                .collect();
    };

    ( mut $hash_name:ident, $from_type:ty, $to_type:ty, $(($key:expr, $value:expr)),* ) => {
        let mut $hash_name: std::collections::HashMap<from_type, to_type> = [ $( ($key, $value) )* ]
                                                                                .iter()
                                                                                .cloned()
                                                                                .collect();
    };
}