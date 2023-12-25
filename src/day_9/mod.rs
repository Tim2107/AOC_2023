use crate::day_9::parser::parse_sequences_from_file;
use crate::day_9::sequence_analysis::{analyze_and_predict_next_numbers, analyze_and_predict_preceding_numbers, sum_of_predictions};
use anyhow::{Result};

mod parser;
mod errors;
mod sequence_analysis;

pub fn solve_day_9() -> Result<(i32,i32)> {
    let sequences = parse_sequences_from_file("resources/input_day_9.txt")?;
    let extrapolated_next_numbers = analyze_and_predict_next_numbers(&sequences);
    let extrapolated_preceding_numbers = analyze_and_predict_preceding_numbers(&sequences);

    let part_1_sum = sum_of_predictions(&extrapolated_next_numbers);
    let part_2_sum = sum_of_predictions(&extrapolated_preceding_numbers);
    Ok((part_1_sum, part_2_sum))
}
