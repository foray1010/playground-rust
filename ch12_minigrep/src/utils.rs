use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn read_file_to_string(filename: &str) -> Result<String, Box<Error>> {
    let mut contents = String::new();
    File::open(filename)?.read_to_string(&mut contents)?;

    Ok(contents)
}
