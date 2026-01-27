fn main() {
    let mut x = 42;
    let x: *mut i32 = &mut x;
    let (x1, x2) = unsafe { (&mut *x, &mut *x) };
    println!("{} {}", x1, x2);
}

// cargo +nightly miri run --example ch6_miri
