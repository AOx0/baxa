use crate::error::Error;

use std::fs::OpenOptions;
use std::io::prelude::*;

/// Returns a Result with either
/// an ` Ok(String) ` status with the contents of the file
/// or an ` Err(Error) ` status with the error wrapped as a ` String ` in ` Error `
pub fn load_from(path: &str) -> Result<String, Error> {
    let mut file_contents: Vec<u8> = Vec::new();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open(path)?;
  
    file.read_to_end(&mut file_contents)?;
    Ok(String::from_utf8(file_contents)?)
}

pub fn write_contents_to(path: &str, contents: &[u8]) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .append(false)
        .open(path)?;

    file.write(contents)?;
    Ok(())
}

pub fn get_file(path: &str) -> Result<std::fs::File, Error> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .append(false)
        .open(path)?;
    Ok(file)
}
