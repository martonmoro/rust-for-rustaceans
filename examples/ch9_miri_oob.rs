fn main() {
    let arr = [1, 2, 3];
    let ptr = arr.as_ptr();

    // UB: out of bounds read
    let val = unsafe { *ptr.add(10) };
    println!("{}", val);
}

// cargo +nightly miri run --example ch9_miri_oob
