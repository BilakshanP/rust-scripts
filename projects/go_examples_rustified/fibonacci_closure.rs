fn fib() -> impl FnMut() -> i32 {
    let mut a = 0;
    let mut b = 1;
    move || {
        let temp = a;
        a = b;
        b += temp;
        temp
    }
}

fn run() {
    let mut f = fib();
    println!("{}, {}, {}, {}, {}", f(), f(), f(), f(), f());
}