#![feature(never_type)]

use std::io;

fn always_ok() -> Result<i32, !> {
    Ok(42)
    // can't do return Err(???)
}

fn run_server() -> Result<!, io::Error> {
    Err(io::Error::new(io::ErrorKind::Other, "server crashed"))

    // in real code
    // loop {
    //     handle_connection()?;
    // }
}

fn main() {
    let val = match always_ok() {
        Ok(v) => v,
        // Err(e) => ...  // not needed
    };
    println!("always_ok returned: {}", val);

    println!("run_server result: {:?}", run_server());
}
