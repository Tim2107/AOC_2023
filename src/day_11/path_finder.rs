use std::collections::HashMap;

pub struct PathFinder{
    pub raw_cosmos_data: Vec<Vec<(char,usize, usize)>>,
    pub galaxy_cataloge: HashMap<usize,(usize,usize)>
}

impl PathFinder {
    pub fn new(cosmos_data: (&Vec<Vec<(char,usize,usize)>>, HashMap<usize, (usize, usize)>)) -> Self {

        let raw_cosmos_data = cosmos_data.0.to_vec();
        let galaxy_cataloge = cosmos_data.1;

        Self { raw_cosmos_data, galaxy_cataloge }
    }

    pub fn calculate_sum_of_shortest_distances(&self) -> usize {
        let mut sum = 0;
        let num_galaxies = self.galaxy_cataloge.len();

        for i in 1..=num_galaxies {
            for j in i + 1..=num_galaxies {
                if let (Some(&galaxy_a), Some(&galaxy_b)) = (self.galaxy_cataloge.get(&i), self.galaxy_cataloge.get(&j)) {
                    let distance = PathFinder::calculate_shortest_distance(galaxy_a, galaxy_b);
                    sum += distance;
                }
            }
        }

        sum
    }

    pub fn calculate_shortest_distance(galaxy_a: (usize, usize) , galaxy_b: (usize, usize) ) -> usize {
        let x_distance = if galaxy_a.0 > galaxy_b.0 { galaxy_a.0 - galaxy_b.0 } else { galaxy_b.0 - galaxy_a.0 };
        let y_distance = if galaxy_a.1 > galaxy_b.1 { galaxy_a.1 - galaxy_b.1 } else { galaxy_b.1 - galaxy_a.1 };

        x_distance + y_distance
    }
}

#[cfg(test)]
mod tests{
    use num::ToPrimitive;
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
    #[case(5,9,9)]
    #[case(1,7,15)]
    #[case(3,6,17)]
    #[case(8,9,5)]
    pub fn test_calculate_shortest_distance( #[case] galaxy_1:usize, #[case] galaxy_2:usize, #[case]shortest_path: usize){
        let observatory_data = read_file("resources/input_day_11_test_a.txt").unwrap();
        let mut parser = Parser::new(&observatory_data,2);
        let cosmos_data = parser.expanded_cosmos_data();
        let path_finder = PathFinder::new(cosmos_data);
        if let (Some(galaxy_a), Some(galaxy_b)) = (path_finder.galaxy_cataloge.get(&galaxy_1), path_finder.galaxy_cataloge.get(&galaxy_2)) {
            let calculated_path = PathFinder::calculate_shortest_distance(*galaxy_a, *galaxy_b);
            assert_eq!(calculated_path, shortest_path);
        } else {
            println!("Oooops, no galaxy data found....")
        }
    }

    #[rstest]
    #[case(2,374)]
    #[case(10,1030)]
    #[case(100,8410)]
    pub fn calculate_sum_of_shortest_distances(#[case] expansion_rate:usize, #[case] expected_distance:usize) {
        let observatory_data = read_file("resources/input_day_11_test_a.txt").unwrap();
        let mut parser = Parser::new(&observatory_data, expansion_rate);
        let cosmos_data = parser.expanded_cosmos_data();
        let path_finder = PathFinder::new(cosmos_data);
        println!("{:?}", path_finder.galaxy_cataloge.get(&7.to_usize().unwrap()));
        let sum_of_distances = path_finder.calculate_sum_of_shortest_distances();
        assert_eq!(sum_of_distances, expected_distance);
    }
}
