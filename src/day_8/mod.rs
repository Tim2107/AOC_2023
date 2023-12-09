mod node;
mod graph;
mod parser;

use parser::{read_file, parse_waypoint_instructions, parse_nodes};
use crate::day_8::graph::Graph;

pub fn solve_day_8_part_1() -> usize {
    let input_file = "resources/input_day_8.txt";

    let file_content = match read_file(input_file) {
        Ok(content) => content,
        Err(_) => panic!("Reading file failed"),
    };

    let waypoint_instructions = match parse_waypoint_instructions(&file_content) {
        Ok(instructions) => instructions,
        Err(_) => panic!("Cycle instructions parsing failed"),
    };

    let nodes = match parse_nodes(&file_content) {
        Ok(nodes) => nodes,
        Err(_) => panic!("Node parsing failed"),
    };

    let mut graph = graph::Graph::new();
    for node in nodes {
        graph.add_node(node);
    }

    let traversal_steps = match graph.traverse("AAA", "ZZZ", &waypoint_instructions, 10000) {
        Some(result) => result,
        None => panic!("Traversal failed"),
    };

    traversal_steps.1
}

pub fn solve_day_8_part_2() -> usize {
    let file_content = read_file("resources/input_day_8.txt")
        .expect("Failed to read test file");

    let waypoint_instructions = parse_waypoint_instructions(&file_content)
        .expect("Failed to parse waypoint instructions");

    let nodes = parse_nodes(&file_content)
        .expect("Failed to parse nodes");

    let mut graph = Graph::new();
    for node in nodes {
        graph.add_node(node);
    }

    let steps_to_simultaneous_end_nodes = graph.find_overall_step(&waypoint_instructions);

    steps_to_simultaneous_end_nodes
}
