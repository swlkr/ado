use std::fs::OpenOptions;
use std::io::{Read, Result};

const FILENAME: &str = "./TODO.txt";

fn main() -> Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(FILENAME)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);

    Ok(())
}
