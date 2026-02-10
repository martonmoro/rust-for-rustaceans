fn main() {
    let mut x: i32 = 42;
    let ptr1 = &mut x as *mut i32;
    let ptr2 = &mut x as *mut i32;

    // UB: writing through two mutable raw pointers derived from separate borrows
    unsafe {
        *ptr1 = 1;
        *ptr2 = 2;
        println!("{}", *ptr1);
    }
}

// cargo +nightly miri run --example ch9_miri_mutablepointers
