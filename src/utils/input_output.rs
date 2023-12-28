use std::path::Path;
use std::{fs, io};

pub fn read_file<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
    fs::read_to_string(file_path)
}

pub fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for &character in row {
            print!("{}", character);
        }
        println!();
    }
}

pub fn print_grid_for_tuples(grid: &Vec<Vec<(char,usize,usize)>>) {
    for row in grid {
        for &tuple in row {
            print!("{:?}", tuple);
        }
        println!();
    }
}
