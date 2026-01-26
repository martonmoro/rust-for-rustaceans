use std::error::Error;
use std::io;
use std::num::ParseIntError;

fn do_stuff() -> Result<(), Box<dyn Error>> {
    let _file = std::fs::File::open("config.txt")?; // io::Error
    let _num: i32 = "not a number".parse()?; // ParseIntError

    Ok(())
}

fn handle_error(err: &(dyn Error + 'static)) {
    if let Some(io_error) = err.downcast_ref::<io::Error>() {
        println!("IO error, kind: {:?}", io_error.kind());
    } else if let Some(parse_err) = err.downcast_ref::<ParseIntError>() {
        println!("Parse error, kind: {:?}", parse_err.kind());
    } else {
        println!("Some other error: {}", err);
    }
}

fn main() {
    let err = do_stuff().unwrap_err();
    println!("Error: {}", err);
    handle_error(err.as_ref());
}
