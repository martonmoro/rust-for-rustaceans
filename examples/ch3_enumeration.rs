use std::error::Error;
use std::fmt::{self, Debug, Display};
use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
pub enum CopyError {
    ReadError(io::Error),
    WriteError(io::Error),
}

impl Error for CopyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CopyError::ReadError(e) => Some(e),
            Self::WriteError(e) => Some(e),
        }
    }
}

impl Display for CopyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CopyError::ReadError(_) => write!(f, "failed to read from source"),
            CopyError::WriteError(_) => write!(f, "failed to write to destination"),
        }
    }
}

fn read_file(path: &str) -> Result<String, CopyError> {
    let mut file = File::open(path).map_err(CopyError::ReadError)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(CopyError::ReadError)?;
    Ok(contents)
}

fn main() {
    match read_file("nonexistent.txt") {
        Ok(contents) => println!("Contents: {}", contents),
        Err(e) => {
            println!("=== Display ===");
            println!("{}\n", e);

            println!("=== Debug ===");
            println!("{:?}\n", e);

            println!("\n=== Proving Send + Sync + 'static ===");
            let handle = std::thread::spawn(move || {
                println!("Error received in another thread: {}", e);
            });
            handle.join().unwrap();
        }
    }
}
