pub struct Parser{
    cosmos: Vec<Vec<char>>,
}

impl Parser {
    pub fn new(observatory_data: &str) -> Self {
        Parser{
            cosmos: observatory_data.lines()
                                    .map(|line| line.chars().collect())
                                    .collect(),
        }
    }
    pub fn adjust_for_cosmic_expansion(&mut self) -> &Vec<Vec<char>> {
        self.double_rows_of_empty_space();
        self.double_columns_of_empty_space();
        &self.cosmos
    }

    fn double_rows_of_empty_space(&mut self) {
        let mut row = 0;
        while row < self.cosmos.len() {
            if self.cosmos[row].iter().all(|&column| column == '.') {
                self.cosmos.insert(row + 1, self.cosmos[row].clone());
                row += 1;
            }
            row += 1;
        }
    }

    fn double_columns_of_empty_space(&mut self) {
        let mut column = 0;
        while column < self.cosmos[0].len() {
            if self.cosmos.iter().all(|row| row[column] == '.') {
                for row in &mut self.cosmos {
                    row.insert(column + 1, '.');
                }
                column += 1;
            }
            column += 1;
        }
    }
}

#[cfg(test)]
mod tests{
    use rstest::rstest;
    use super::*;
    use crate::utils::input_output::{read_file};

    #[rstest]
    #[case("resources/input_day_11_test_a.txt")]
    #[case("resources/input_day_11_test_b.txt")]
    #[case("resources/input_day_11_test_c.txt")]
    #[case("resources/input_day_11_test_d.txt")]
    fn test_read_file(#[case] input_file: &str) {
        let content = read_file(input_file).unwrap();
        assert!(!content.is_empty());
    }

    #[test]
    fn test_adjust_for_cosmic_expansion() {
        let observatory_data = read_file("resources/input_day_11_test_a.txt").unwrap();
        let mut parser = Parser::new(&observatory_data);
        let expanded_cosmos = parser.adjust_for_cosmic_expansion();
        let expected_cosmos_data = read_file("resources/input_day_11_test_b.txt").unwrap();
        let expected_data_parser = Parser::new(&expected_cosmos_data);
        let expected_cosmos = expected_data_parser.cosmos;
        assert_eq!(*expanded_cosmos, expected_cosmos);
    }
}
