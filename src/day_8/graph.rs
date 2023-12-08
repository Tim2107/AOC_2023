use std::collections::HashMap;
use crate::day_8::node::Node;

#[derive(Debug)]
pub enum TraversalError {
    InvalidInstruction,
    NodeNotFound,
    CycleLimitReached(usize),
}

pub struct Graph {
    nodes: HashMap<String, Node>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.name().to_string(), node);
    }

    pub fn traverse(&self, start: &str, target: &str, waypoint_instructions: &str, max_cycles: usize) -> Option<(String, usize)> {
        let mut current_node = start;
        let mut current_cycle = 0;
        let mut steps = 0;
        let instruction_len = waypoint_instructions.chars().count();

        while current_cycle < max_cycles {
            for i in 0..instruction_len {
                if let Some(node) = self.nodes.get(current_node) {
                    if current_node == target {
                        return Some((target.to_string(), steps));
                    }

                    let next_instruction = waypoint_instructions.chars().nth(i).unwrap();
                    current_node = match next_instruction {
                        'R' => node.right_neighbour(),
                        'L' => node.left_neighbour(),
                        _ => return None,
                    };
                    steps += 1;
                } else {
                    return None;
                }
            }
            current_cycle += 1;
        }
        None
    }

    pub fn traverse_all(&self, waypoint_instructions: &str, max_cycles: usize) -> Result<usize, TraversalError> {
        let start_nodes: Vec<_> = self.nodes.values()
            .filter(|node| node.is_start_node())
            .collect();

        let mut paths = start_nodes;
        let mut current_cycle = 0;
        let instruction_len = waypoint_instructions.chars().count();

        while current_cycle < max_cycles {
            for i in 0..instruction_len {
                let mut next_paths = Vec::new();
                let instruction = waypoint_instructions.chars().nth(i).unwrap();

                for node in &paths {
                    let next_node = match instruction {
                        'R' => self.nodes.get(node.right_neighbour()),
                        'L' => self.nodes.get(node.left_neighbour()),
                        _ => return Err(TraversalError::InvalidInstruction),
                    };

                    if let Some(next_node) = next_node {
                        next_paths.push(next_node);
                    } else {
                        return Err(TraversalError::NodeNotFound);
                    }
                }

                if next_paths.iter().all(|node| node.is_target_node()) {
                    return Ok(current_cycle * instruction_len + i + 1);
                }

                paths = next_paths;
            }

            current_cycle += 1;
        }

        Err(TraversalError::CycleLimitReached(current_cycle))
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::day_8::parser::{read_file, parse_waypoint_instructions, parse_nodes};

    #[test]
    fn testcase_a(){
        test_add_and_traverse_graph_helper("resources/input_day_8_test_a.txt",2)
    }

    #[test]
    fn testcase_b(){
        test_add_and_traverse_graph_helper("resources/input_day_8_test_b.txt",6)
    }

    fn test_add_and_traverse_graph_helper(testfile_path: &str, expected_steps: usize ) {
        let file_content = read_file(testfile_path)
            .expect("Failed to read test file");

        let waypoint_instructions = parse_waypoint_instructions(&file_content)
            .expect("Failed to parse waypoint instructions");

        let nodes = parse_nodes(&file_content)
            .expect("Failed to parse nodes");

        let mut graph = Graph::new();
        for node in nodes {
            graph.add_node(node);
        }

        let start = "AAA";
        let target = "ZZZ";
        let max_cycles = 10;

        let traversal_result = graph.traverse(start, target, &waypoint_instructions, max_cycles)
            .expect("Traversal failed");

        assert_eq!(traversal_result.1, expected_steps);
    }

    #[test]
    fn test_traverse_all() {
        let file_content = read_file("resources/input_day_8_test_c.txt")
            .expect("Failed to read test file");

        let waypoint_instructions = parse_waypoint_instructions(&file_content)
            .expect("Failed to parse waypoint instructions");

        let nodes = parse_nodes(&file_content)
            .expect("Failed to parse nodes");

        let mut graph = Graph::new();
        for node in nodes {
            graph.add_node(node);
        }

        let max_cycles = 10;

        let traversal_result = graph.traverse_all( &waypoint_instructions, max_cycles)
            .expect("Traversal failed");

        assert_eq!(traversal_result, 6);
    }
}

