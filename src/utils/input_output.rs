use std::path::Path;
use std::{fs, io};

pub fn read_file<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
    fs::read_to_string(file_path)
}
