use std::any::TypeId;
use std::marker::PhantomData;

trait TypeEq {
    fn same_type<T: 'static>(&self) -> bool;
}

impl<T: 'static> TypeEq for PhantomData<T> {
    fn same_type<U: 'static>(&self) -> bool {
        TypeId::of::<T>() == TypeId::of::<U>()
    }
}

fn assert_same_type<T: 'static, U: 'static>() {
    let t = PhantomData::<T>;
    assert!(t.same_type::<U>(), "Types are not the same!");
}

fn main() {
    assert_same_type::<i32, i32>();
    assert_same_type::<i32, f64>();
}
