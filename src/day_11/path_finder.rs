use std::collections::HashMap;

pub struct PathFinder{
    pub raw_cosmos_data: Vec<Vec<char>>,
    pub galaxy_cataloge: HashMap<usize,(usize,usize)>
}

impl PathFinder {
    pub fn new(cosmos_data: (&Vec<Vec<char>>, HashMap<usize, (usize, usize)>)) -> Self {

        let raw_cosmos_data = cosmos_data.0.to_vec();
        let galaxy_cataloge = cosmos_data.1;

        Self { raw_cosmos_data, galaxy_cataloge }
    }

    pub fn calculate_shortest_distance(galaxy_a: (usize, usize) , galaxy_b: (usize, usize) ) -> usize {
        let x_distance = if galaxy_a.0 > galaxy_b.0 { galaxy_a.0 - galaxy_b.0 } else { galaxy_b.0 - galaxy_a.0 };
        let y_distance = if galaxy_a.1 > galaxy_b.1 { galaxy_a.1 - galaxy_b.1 } else { galaxy_b.1 - galaxy_a.1 };

        x_distance + y_distance
    }
}

#[cfg(test)]
mod tests{
    use rstest::rstest;
    use crate::day_11::parser::Parser;
    use crate::utils::input_output::read_file;
    use super::*;

    #[rstest]
    #[case("resources/input_day_11_test_a.txt")]
    #[case("resources/input_day_11_test_b.txt")]
    #[case("resources/input_day_11_test_c.txt")]
    #[case("resources/input_day_11_test_d.txt")]

    fn test_read_file(#[case] input_file: &str) {
        let content = read_file(input_file).unwrap();
        assert!(!content.is_empty());
    }

    #[rstest]
    pub fn test_calculate_shortest_distance(){
        let observatory_data = read_file("resources/input_day_11_test_a.txt").unwrap();
        let mut parser = Parser::new(&observatory_data);
        let cosmos_data = parser.expanded_cosmos_data();
        let path_finder = PathFinder::new(cosmos_data);
        if let (Some(galaxy_a), Some(galaxy_b)) = (path_finder.galaxy_cataloge.get(&5), path_finder.galaxy_cataloge.get(&9)) {
            let calculated_path = PathFinder::calculate_shortest_distance(*galaxy_a, *galaxy_b);
            assert_eq!(calculated_path, 9);
        } else {
            println!("Oooops, no galaxy data found....")
        }
    }
}
