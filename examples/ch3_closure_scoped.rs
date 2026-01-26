use std::error::Error;

fn do_the_thing() -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::create("temp.txt")?;

    let result = || -> Result<i32, Box<dyn Error>> {
        let content = std::fs::read_to_string("config.txt")?;
        Ok(content.trim().parse::<i32>()?)
    }();

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
