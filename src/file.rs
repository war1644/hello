use std::fs::File;
use std::io::{self, Read};

pub struct MyFile;
impl MyFile {
    //读取file
    pub fn read(filename: &str) -> io::Result<String> {
        let mut text = String::new();
        File::open(&filename)?.read_to_string(&mut text)?;
        Ok(text)
    }

    pub fn save(filename: &str,file_data: &str) -> io::Result<String> {
        let mut text = String::new();
        File::open(&filename)?.write(&mut text)?;
        Ok(text)
    }

    pub fn add(filename: &str,file_data: &str) -> io::Result<String> {
        let mut text = String::new();
        File::open(&filename)?.write(&mut text)?;
        Ok(text)
    }

    pub fn log(filename: &str,file_data: &str) -> io::Result<String> {
        let mut text = String::new();
        File::open(&filename)?.write(&mut text)?;
        Ok(text)
    }


}
