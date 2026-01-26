#![feature(try_blocks)]

use std::error::Error;

fn do_the_thing() -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::create("temp.txt")?;

    let result: Result<i32, Box<dyn Error>> = try {
        let content =
            std::fs::read_to_string("config.txt").map_err(|e| Box::new(e) as Box<dyn Error>)?;
        content
            .trim()
            .parse::<i32>()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?
    };

    // Cleanup always runs, even if the try block failed
    drop(file);
    std::fs::remove_file("temp.txt")?;
    println!("Cleanup done!");

    let port = result?;
    println!("Port: {}", port);
    Ok(())
}

fn main() {
    match do_the_thing() {
        Ok(()) => println!("Success"),
        Err(e) => println!("Error: {}", e),
    }
}
