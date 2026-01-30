use ch7_proc_macro_impl::{inc_var, make_fn};

make_fn!();

fn main() {
    generated_fn();

    let mut a = 10;
    inc_var!(a); // expands to `a += 1`
    assert_eq!(a, 11);
}
