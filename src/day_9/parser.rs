use anyhow::{Result, Context};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn parse_sequences_from_file(file_path: &str) -> Result<Vec<Vec<i32>>> {
    let path = Path::new(file_path);
    let file = File::open(&path).context("Failed to open file")?;
    let reader = io::BufReader::new(file);

    reader.lines()
        .map(|line| {
            line.context("Failed to read line")
                .and_then(|l| {
                    l.split_whitespace()
                        .map(|num| num.parse::<i32>().context("Failed to parse number"))
                        .collect()
                })
        })
        .collect()
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::utils::input_output::read_file;

    #[test]
    fn test_read_test_file(){
        let content = read_file("resources/input_day_9_test.txt").unwrap();
        assert!(!content.is_empty());
    }

    #[test]
    fn test_parse_sequences_from_file(){
        let sequences = parse_sequences_from_file("resources/input_day_9_test.txt").unwrap();
        let expected_sequence = vec![0, 3, 6, 9, 12, 15];
        assert!(sequences.contains(&expected_sequence));
    }
}
