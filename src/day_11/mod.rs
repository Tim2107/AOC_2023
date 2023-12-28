use crate::utils::input_output::read_file;
use crate::day_11::parser::Parser;
use crate::day_11::path_finder::PathFinder;

mod parser;
mod path_finder;

pub fn solve_day_11() -> anyhow::Result<(usize, usize)> {
    let content = read_file("resources/input_day_11.txt")?;

    let mut parser = Parser::new(&content,2);
    let cosmos_data = parser.expanded_cosmos_data();
    let path_finder = PathFinder::new(cosmos_data);
    let sum_of_distances_expansion_rate_2 = path_finder.calculate_sum_of_shortest_distances();

    let mut parser = Parser::new(&content,1000000);
    let cosmos_data = parser.expanded_cosmos_data();
    let path_finder = PathFinder::new(cosmos_data);
    let sum_of_distances_expansion_rate_1000000 = path_finder.calculate_sum_of_shortest_distances();

    Ok((sum_of_distances_expansion_rate_2, sum_of_distances_expansion_rate_1000000))
}
