use std::fs::File;
use std::io::{self, Read};

//读取file
pub fn read_file_data(filename: &str) -> io::Result<String> {
    let mut text = String::new();
    File::open(&filename)?.read_to_string(&mut text)?;
    Ok(text)
}

pub fn save_file_data(filename: &str,file_data: &str) -> io::Result<String> {
    let mut text = String::new();
    File::open(&filename)?.write(&mut text)?;
    Ok(text)
}