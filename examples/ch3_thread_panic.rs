// examples/ch03_thread_panic.rs

fn main() {
    let handle = std::thread::spawn(|| {
        panic!("oops!");
    });

    // join() returns Result<T, Box<dyn Any + Send>>
    match handle.join() {
        Ok(_) => println!("Thread completed"),
        Err(panic_value) => {
            if let Some(s) = panic_value.downcast_ref::<&str>() {
                println!("Thread panicked with: {}", s);
            } else if let Some(s) = panic_value.downcast_ref::<String>() {
                println!("Thread panicked with: {}", s);
            } else {
                println!("Thread panicked with unknown type");
            }
        }
    }
}
