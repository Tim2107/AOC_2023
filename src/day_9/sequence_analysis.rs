pub fn sum_of_predictions(predictions: &[i32]) -> i32 {
    predictions.iter().sum()
}

pub fn analyze_and_predict_next_numbers(sequences: &[Vec<i32>]) -> Vec<i32> {
    analyze_and_predict_numbers(sequences, extrapolate_next_number)
}

pub fn analyze_and_predict_preceding_numbers(sequences: &[Vec<i32>]) -> Vec<i32> {
    analyze_and_predict_numbers(sequences, extrapolate_preceeding_number)
}

pub fn analyze_and_predict_numbers<F>(sequences: &[Vec<i32>], extrapolate: F) -> Vec<i32>
    where
        F: Fn(&[Vec<i32>]) -> i32
{
    sequences
        .into_iter()
        .map(|sequence| {
            let difference_sequences = calculate_complete_difference_sequences(&sequence);
            extrapolate(&difference_sequences)
        })
        .collect()
}

pub fn extrapolate_next_number(sequences: &[Vec<i32>]) -> i32 {
    let mut next_number = 0;
    for sequence in sequences.iter().rev() {
        next_number += sequence.last().copied().unwrap_or_default();
    }

    next_number
}

pub fn extrapolate_preceeding_number(sequences: &[Vec<i32>]) -> i32 {
    let mut preceeding_number = 0;
    for sequence in sequences.iter().rev() {
        preceeding_number   = sequence.first().copied().unwrap_or_default() - preceeding_number;
    }

    preceeding_number
}


pub fn calculate_complete_difference_sequences(sequence: &[i32]) -> Vec<Vec<i32>> {
    let mut sequences = vec![sequence.to_vec()];
    let mut current_sequence = sequence.to_vec();

    while current_sequence.iter().any(|&x| x != 0) {
        current_sequence = calculate_differences(&current_sequence);
        sequences.push(current_sequence.clone());
    }

    sequences
}

pub fn calculate_differences(sequence: &[i32]) -> Vec<i32> {
    sequence.windows(2)
        .map(|window| window[1] - window[0])
        .collect()
}

#[cfg(test)]
mod tests{
    use crate::day_9::parser::parse_sequences_from_file;
    use super::*;

    #[test]
    fn test_calculate_differences(){
        let expected_differences = vec![3,3,3,3,3];

        let sequences_to_analyze = parse_sequences_from_file("resources/input_day_9_test.txt").unwrap();
        let calculated_differences = calculate_differences(sequences_to_analyze.first().unwrap());

        assert_eq!(calculated_differences, expected_differences);
    }

    #[test]
    fn test_calculate_complete_difference_sequences(){
        let original_sequence = vec![0,3,6,9,12,15];
        let expected_differences_1 = vec![3,3,3,3,3];
        let expected_differences_2 = vec![0,0,0,0];

        let sequences_to_analyze = parse_sequences_from_file("resources/input_day_9_test.txt").unwrap();
        let calculated_sequences = calculate_complete_difference_sequences(sequences_to_analyze.first().unwrap());

        assert_eq!(calculated_sequences.len(),3);
        assert!(calculated_sequences.contains(&original_sequence));
        assert!(calculated_sequences.contains(&expected_differences_1));
        assert!(calculated_sequences.contains(&expected_differences_2));
    }

    #[test]
    fn test_extrapolate_next_number_a(){
        test_extrapolate_next_number_helper(0,18);
    }

    #[test]
    fn test_extrapolate_next_number_b(){
        test_extrapolate_next_number_helper(1,28);
    }

    #[test]
    fn test_extrapolate_next_number_c(){
        test_extrapolate_next_number_helper(2,68);
    }

    fn test_extrapolate_next_number_helper(test_sequence: usize, expected_result: i32){
        let sequences_to_analyze = parse_sequences_from_file("resources/input_day_9_test.txt").unwrap();
        let calculated_sequences = calculate_complete_difference_sequences(&sequences_to_analyze[test_sequence]);
        let extrapolated_number = extrapolate_next_number(&calculated_sequences);
        assert_eq!(extrapolated_number, expected_result);
     }

    #[test]
    fn test_extrapolate_preceeding_number_a(){test_extrapolate_preceeding_number_helper(0, -3);
    }

    #[test]
    fn test_extrapolate_preceeding_number_b(){test_extrapolate_preceeding_number_helper(1, 0);
    }

    #[test]
    fn test_extrapolate_preceeding_number_c(){test_extrapolate_preceeding_number_helper(2, 5);
    }

    fn test_extrapolate_preceeding_number_helper(test_sequence: usize, expected_result: i32){
        let sequences_to_analyze = parse_sequences_from_file("resources/input_day_9_test.txt").unwrap();
        let calculated_sequences = calculate_complete_difference_sequences(&sequences_to_analyze[test_sequence]);
        let extrapolated_number = extrapolate_preceeding_number(&calculated_sequences);
        assert_eq!(extrapolated_number, expected_result);
    }

    #[test]
    fn test_analyze_and_predict_next_numbers(){
        let expected_numbers = vec![18,28,68];
        let sequences_to_analyze = parse_sequences_from_file("resources/input_day_9_test.txt").unwrap();
        let next_numbers = analyze_and_predict_next_numbers(&sequences_to_analyze);
        assert_eq!(next_numbers, expected_numbers);
    }

    #[test]
    fn test_analyze_and_predict_preceeding_numbers(){
        let expected_numbers = vec![-3,0,5];
        let sequences_to_analyze = parse_sequences_from_file("resources/input_day_9_test.txt").unwrap();
        let preceeding_numbers = analyze_and_predict_preceding_numbers(&sequences_to_analyze);
        assert_eq!(preceeding_numbers, expected_numbers);
    }

    #[test]
    fn test_sum_of_predictions(){
        let sequences_to_analyze = parse_sequences_from_file("resources/input_day_9_test.txt").unwrap();
        let next_numbers = analyze_and_predict_next_numbers(&sequences_to_analyze);
        let sum_of_extrapolated_numbers = sum_of_predictions(&next_numbers);
        assert_eq!(sum_of_extrapolated_numbers, 114);
    }
}
