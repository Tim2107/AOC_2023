use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn solve_day_1() -> io::Result<i32> {
    let path = "resources/input_day_1.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let word_to_digit = create_word_to_digit_map();

    let mut total_sum = 0;

    for line in reader.lines() {
        let line = line?;
        let extracted_number = extract_number(&line, &word_to_digit);
        let new_number = first_and_last(&extracted_number);

        if let Ok(num) = new_number.parse::<i32>() {
            total_sum += num;
        }
    }
    Ok(total_sum)
}

fn extract_number(line: &str, word_to_digit: &HashMap<String, char>) -> String {
    let mut result = String::new();
    let mut current_word = String::new();

    for char in line.chars() {
        if char.is_digit(10) {
            result.push(char);
            current_word.clear();
        } else if char.is_alphabetic() {
            current_word.push(char.to_ascii_lowercase());
            for j in 1..=5 {
                if current_word.len() >= j {
                    let substring = &current_word[current_word.len() - j..];
                    if let Some(&digit) = word_to_digit.get(substring) {
                        result.push(digit);
                        break;
                    }
                }
            }
        } else {
            current_word.clear();
        }
    }

    result
}

fn first_and_last(number: &str) -> String {
    if number.is_empty() {
        return String::new();
    }

    let first_digit = number.chars().next().unwrap();
    let last_digit = number.chars().last().unwrap_or(first_digit);
    format!("{}{}", first_digit, last_digit)
}

fn create_word_to_digit_map() -> HashMap<String, char> {
    let words = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    words.iter().enumerate().map(|(i, &word)| (word.to_string(), char::from_digit(i as u32, 10).unwrap())).collect()
}