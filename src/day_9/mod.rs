use crate::day_9::parser::parse_sequences_from_file;
use crate::day_9::sequence_analysis::{analyze_and_predict_next_numbers, sum_of_predictions};
use anyhow::{Result};

mod parser;
mod errors;
mod sequence_analysis;

pub fn solve_day_9() -> Result<i32> {
    let sequences = parse_sequences_from_file("resources/input_day_9.txt")?;
    let extrapolated_numbers = analyze_and_predict_next_numbers(sequences);

    let part_1_sum = sum_of_predictions(&extrapolated_numbers);
    Ok(part_1_sum)
}