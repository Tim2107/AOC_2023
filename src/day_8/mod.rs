mod node;
mod graph;
mod parser;
mod errors;

use crate::utils::input_output::read_file;
use graph::Graph;

pub fn solve_day_8() -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let graph_input = read_file("resources/input_day_8.txt")?;
    let graph = Graph::new(&graph_input)?;

    let part_1_steps = graph.traverse("AAA", "ZZZ", 10000)?;
    let part_2_steps = graph.find_overall_step()?;

    Ok((part_1_steps, part_2_steps))
}


