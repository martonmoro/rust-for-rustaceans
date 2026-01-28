macro_rules! let_foo {
    ($val:expr) => {
        let foo = $val;
    };
}

fn main() {
    let foo = 1;
    let_foo!(2);
    assert_eq!(foo, 1);
}
