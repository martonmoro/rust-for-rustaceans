use std::{
    error::Error,
    fmt::{Display, write},
};

fn might_fail_unit() -> Result<i32, ()> {
    Err(())
}

#[derive(Debug)]
struct ValidationFailed;

impl Display for ValidationFailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "validation failed")
    }
}

impl Error for ValidationFailed {}

fn might_fail_better() -> Result<i32, ValidationFailed> {
    Err(ValidationFailed)
}

fn uses_dyn_error() -> Result<i32, Box<dyn Error>> {
    let val = might_fail_better()?;
    // let val = might_fail_unit()?; // does not compile: () doesn't implement Error
    Ok(val)
}

fn main() {
    println!("=== Unit error vs custom error ===");
    println!("might_fail_unit: {:?}", might_fail_unit());
    println!("might_fail_better: {:?}", might_fail_better());
    println!("uses_dyn_error: {:?}", uses_dyn_error());
}
