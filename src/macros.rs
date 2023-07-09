#[macro_export]
macro_rules! _test {
    ( $e:expr ) => {
        println!("{}={:?}", stringify!($e), $e)
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

#[macro_export]
macro_rules! r#mod {
    ( $( $module_name:ident ),* ) => {
        $( mod $module_name; )*
    };

    ( pub $( $module_name:ident ),* ) => {
        $( pub mod $module_name; )*
    }
}

#[macro_export]
macro_rules! r#use {
    ( $( $module_path:path ),* ) => {
        $( use $module_path; )*
    };
}

#[macro_export]
macro_rules! mutate {
    ( $from_var:ident, $to_var:ident, $type:ty ) => {
        let $to_var = &$from_var as *const $type as usize as *mut $type;
    };
}

#[macro_export]
macro_rules! mutate_as {
    ( $from_var:ident, $from_type:ty, $to_var:ident, $to_type:ty ) => {
        let $to_var = &$from_var as *const $from_type as usize as *mut $to_type;
    };
}