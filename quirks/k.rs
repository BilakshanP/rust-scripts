trait Some {
    fn foo(&self, x: i32);
    fn foo_default(&self);
    fn foo_str(&self, s: &str);
}

struct Foo;

impl Foo {
    fn create<T: Some>(some: T) {
        some.foo(42);
        some.foo(42);
        some.foo_str("Hello World");
    }
}

struct SomeType;

impl Some for SomeType {
    fn foo(&self, x: i32) {
        println!("Called foo with {}", x);
    }

    fn foo_default(&self) {
        println!("Called foo_default");
    }

    fn foo_str(&self, s: &str) {
        println!("Called foo_str with {}", s);
    }
}

fn main() {
    let some = SomeType;
    Foo::create(some);
}
