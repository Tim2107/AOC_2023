enum Dimension {
    Row,
    Column,
}

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

    pub fn adjust_for_cosmic_expansion(&mut self) {
        self.double_at_index_if_empty_space(Dimension::Row);
        self.double_at_index_if_empty_space(Dimension::Column);
    }

    fn double_at_index_if_empty_space (&mut self, dimension: Dimension) {
        let mut index : usize = 0;
        while match dimension { Dimension::Row =>       self.is_below_row_size(index),
                                Dimension::Column =>    self.is_below_column_size(index) }
        { if match dimension  { Dimension::Row =>       self.is_empty_row(index),
                                Dimension::Column =>    self.is_empty_column(index) }
            {
                match dimension  { Dimension::Row =>       self.double_row(index),
                                   Dimension::Column =>    self.double_column(index) }
                index +=1;
            }
            index +=1;
        }

    }

    fn is_empty_row(&self, row: usize) -> bool {
        self.cosmos[row].iter().all(|&column| column == '.')
    }

    fn is_empty_column(&self, column: usize) -> bool {
        self.cosmos.iter().all(|row| row[column] == '.')
    }

    fn double_row(&mut self, row: usize) {
        self.cosmos.insert(row + 1, self.cosmos[row].clone());
    }

    fn double_column(&mut self, column: usize) {
        for row in &mut self.cosmos {
            row.insert(column + 1, '.');
        }
    }

    fn is_below_row_size(&self, row: usize) -> bool {
        row < self.cosmos.len()
    }

    fn is_below_column_size(&self, column: usize) -> bool {
        column < self.cosmos[0].len()
    }

    fn catalogue_galaxies(&mut self){
        
    }
}

#[cfg(test)]
mod tests{
    use rstest::*;
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

    #[fixture]
    pub fn test_cosmos() -> String {
        "..#\n...\n#..".to_string()
    }

    #[rstest]
    pub fn test_is_empty_row_positive(test_cosmos: String) {
        let parser = Parser::new(&test_cosmos);
        assert!(parser.is_empty_row(1));
    }

    #[rstest]
    pub fn test_is_empty_row_negative(test_cosmos: String) {
        let parser = Parser::new(&test_cosmos);
        assert!(!parser.is_empty_row(0));
    }

    #[rstest]
    pub fn test_is_empty_column_positive(test_cosmos: String) {
        let parser = Parser::new(&test_cosmos);
        assert!(parser.is_empty_column(1));
    }

    #[rstest]
    pub fn test_is_empty_column_negative(test_cosmos: String) {
        let parser = Parser::new(&test_cosmos);
        assert!(!parser.is_empty_column(0));
    }


    #[rstest]
    pub fn test_double_columns_of_empty_space(test_cosmos: String) {
      let expected_cosmos = vec![vec!['.', '.','.','#'],
                                 vec!['.', '.','.','.'],
                                 vec!['#', '.','.','.']];

        let mut parser = Parser::new(&test_cosmos);
        parser.double_at_index_if_empty_space(Dimension::Column);
        assert_eq!(parser.cosmos, expected_cosmos);
    }

    #[rstest]
    pub fn test_double_rows_of_empty_space(test_cosmos: String) {
        let expected_cosmos = vec![vec!['.', '.', '#'],
                                   vec!['.', '.', '.'],
                                   vec!['.', '.', '.'],
                                   vec!['#', '.', '.']];

        let mut parser = Parser::new(&test_cosmos);
        parser.double_at_index_if_empty_space(Dimension::Row);
        assert_eq!(parser.cosmos, expected_cosmos);
    }

    #[test]
    fn test_adjust_for_cosmic_expansion() {

        let observatory_data = read_file("resources/input_day_11_test_a.txt").unwrap();
        let mut parser = Parser::new(&observatory_data);
        parser.adjust_for_cosmic_expansion();
        let expanded_cosmos = parser.cosmos;

        let expected_cosmos_data = read_file("resources/input_day_11_test_b.txt").unwrap();
        let expected_data_parser = Parser::new(&expected_cosmos_data);
        let expected_cosmos = expected_data_parser.cosmos;

        assert_eq!(expanded_cosmos, expected_cosmos);
    }
}
