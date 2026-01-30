use ch7_proc_macro_impl::Describe;

#[derive(Describe)]
struct Person {
    name: String,
}

fn main() {
    let alice = Person {
        name: "Alice".to_string(),
    };
    println!("{} named {}", alice.describe(), alice.name);
}
