use std::path::Path;
use std::{fs, io};

pub fn read_file<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
    fs::read_to_string(file_path)
}

pub fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for &ch in row {
            print!("{}", ch);
        }
        println!();
    }
}
