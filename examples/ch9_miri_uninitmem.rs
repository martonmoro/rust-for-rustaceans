use std::mem::MaybeUninit;

fn main() {
    let x: MaybeUninit<i32> = MaybeUninit::uninit();

    // UB: reading uninitialized memory
    let val = unsafe { x.assume_init() };
    println!("{}", val);
}

// cargo +nightly miri run --example ch9_miri_uninitmem
