use std::collections::HashMap;
use crate::day_11::rle_compressed_vector_iterator::RLECompressedVectorIterator;

enum Dimension {
    Row,
    Column,
}

pub struct Parser{
    pub cosmos: Vec<Vec<(char, usize, usize)>>,
    expansion_rate: usize
}

impl Parser {
    pub fn new(observatory_data: &str, expansionrate: usize) -> Self {
        Parser{
            cosmos: observatory_data
                    .lines()
                    .map(|line| {line.chars()
                                     .map(|datapoint| (datapoint, 1, 1))
                                     .collect()}
                    )
                    .collect(),
            expansion_rate: expansionrate,
        }
    }

    pub fn expanded_cosmos_data(&mut self) -> (&Vec<Vec<(char, usize, usize)>>, HashMap<usize, (usize, usize)>) {
        self.adjust_for_cosmic_expansion();
        let galaxy_cataloge = self.cataloge_galaxies();
        (&self.cosmos, galaxy_cataloge)
    }
    
    pub fn adjust_for_cosmic_expansion(&mut self) {
        self.multiply_at_index_if_empty_space(Dimension::Row);
        self.multiply_at_index_if_empty_space(Dimension::Column);
    }

    fn multiply_at_index_if_empty_space(&mut self, dimension: Dimension) {
        let mut index: usize = 0;

        while match dimension { Dimension::Row =>       self.is_below_row_size(index),
                                Dimension::Column =>    self.is_below_column_size(index) }

        { if match dimension  { Dimension::Row =>       self.is_empty_row(index),
                                Dimension::Column =>    self.is_empty_column(index) } {

            match dimension   { Dimension::Row =>       self.expand_row(index),
                                Dimension::Column =>    self.expand_column(index) }

            }
            index +=1;
        }
    }

    fn is_empty_row(&self, row: usize) -> bool {
        self.cosmos[row].iter().all(|&(column,_,_)| column == '.')
    }

    fn is_empty_column(&self, column: usize) -> bool {
        self.cosmos.iter().all(|row| row[column].0 == '.')
    }

    fn expand_row(&mut self, row: usize) {
        if let Some(row_data) = self.cosmos.get_mut(row) {
            for (_, _, y_count) in row_data.iter_mut() {
                *y_count *= self.expansion_rate;
            }
        }
    }

    fn expand_column(&mut self, column: usize) {

        for row in &mut self.cosmos {
            if let Some((_, x_count, _)) = row.get_mut(column) {
                *x_count *= self.expansion_rate;
            }
        }
    }

    fn is_below_row_size(&self, row: usize) -> bool {
        row < self.cosmos.len()
    }

    fn is_below_column_size(&self, column: usize) -> bool {
        column < self.cosmos[0].len()
    }

    fn cataloge_galaxies(&mut self) -> HashMap<usize, (usize, usize)> {
        let mut galaxy_positions = HashMap::new();
        let mut galaxy_number = 1;

        for (x, y, data_point) in RLECompressedVectorIterator::new(&self.cosmos) {
            if data_point == '#' {
                galaxy_positions.insert(galaxy_number, (x, y));
                galaxy_number += 1;
            }
        }

        galaxy_positions
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
        let parser = Parser::new(&test_cosmos,2);
        assert!(parser.is_empty_row(1));
    }

    #[rstest]
    pub fn test_is_empty_row_negative(test_cosmos: String) {
        let parser = Parser::new(&test_cosmos,2);
        assert!(!parser.is_empty_row(0));
    }

    #[rstest]
    pub fn test_is_empty_column_positive(test_cosmos: String) {
        let parser = Parser::new(&test_cosmos,2);
        assert!(parser.is_empty_column(1));
    }

    #[rstest]
    pub fn test_is_empty_column_negative(test_cosmos: String) {
        let parser = Parser::new(&test_cosmos,2);
        assert!(!parser.is_empty_column(0));
    }


    #[rstest]
    pub fn test_double_columns_of_empty_space(test_cosmos: String) {
      let expected_cosmos: Vec<Vec<(char,usize, usize)>>=   vec![vec![('.',1,1), ('.',2,1),('#',1,1)],
                                                                 vec![('.',1,1), ('.',2,1),('.',1,1)],
                                                                 vec![('#',1,1), ('.',2,1),('.',1,1)]];

        let mut parser = Parser::new(&test_cosmos,2);
        parser.multiply_at_index_if_empty_space(Dimension::Column);
        assert_eq!(parser.cosmos, expected_cosmos);
    }

    #[rstest]
    pub fn test_double_rows_of_empty_space(test_cosmos: String) {
        let expected_cosmos = vec![vec![('.',1,1), ('.',1,1), ('#',1,1)],
                                   vec![('.',1,2), ('.',1,2), ('.',1,2)],
                                   vec![('#',1,1), ('.',1,1), ('.',1,1)]];

        let mut parser = Parser::new(&test_cosmos,2);
        parser.multiply_at_index_if_empty_space(Dimension::Row);
        assert_eq!(parser.cosmos, expected_cosmos);
    }
    /* TODO: adapt
    #[test]
    fn test_adjust_for_cosmic_expansion() {

        let observatory_data = read_file("resources/input_day_11_test_a.txt").unwrap();
        let mut parser = Parser::new(&observatory_data,2);
        parser.adjust_for_cosmic_expansion();
        let expanded_cosmos = parser.cosmos;

        let expected_cosmos_data = read_file("resources/input_day_11_test_b.txt").unwrap();
        let expected_data_parser = Parser::new(&expected_cosmos_data,2);
        let expected_cosmos = expected_data_parser.cosmos;

        assert_eq!(expanded_cosmos, expected_cosmos);
    }
    */
    #[test]
    fn test_cataloge_galaxies() {

        let observatory_data = read_file("resources/input_day_11_test_b.txt").unwrap();
        let mut parser = Parser::new(&observatory_data,2);
        let catalog = parser.cataloge_galaxies();
        let raw_cosmos_data : Vec<Vec<(char, usize, usize)>> = parser.cosmos;
        let cataloged_cosmos = insert_cataloge_data_into_cosmos(raw_cosmos_data, catalog);
        
        let expected_cosmos_data = read_file("resources/input_day_11_test_c.txt").unwrap();
        let expected_data_parser = Parser::new(&expected_cosmos_data,2);
        let expected_cataloged_cosmos = expected_data_parser.cosmos;

        assert_eq!(cataloged_cosmos, expected_cataloged_cosmos);
    }

    fn insert_cataloge_data_into_cosmos(mut raw_cosmos_data: Vec<Vec<(char, usize, usize)>>, catalogued_galaxies: HashMap<usize, (usize, usize)>) -> Vec<Vec<(char, usize, usize)>> {
        for (number, (x, y)) in catalogued_galaxies {
            if let Some(row) = raw_cosmos_data.get_mut(y) {
                if let Some(column) = row.get_mut(x) {
                    column.0 = char::from_digit(number as u32 % 10, 10).unwrap_or('#');
                }
            }
        }
        raw_cosmos_data
    }
}
