use std::collections::HashMap;
use crate::day_8::node::Node;

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

    pub fn traverse(&self, start: &str, target: &str, cycle_instructions: &str, max_cycles: usize) -> Option<(String, usize)> {
        let mut current_node = start;
        let mut current_cycle = 0;
        let mut steps = 0;
        let instruction_len = cycle_instructions.chars().count();

        while current_cycle < max_cycles {
            for i in 0..instruction_len {
                if let Some(node) = self.nodes.get(current_node) {
                    if current_node == target {
                        return Some((target.to_string(), steps));
                    }

                    let next_instruction = cycle_instructions.chars().nth(i).unwrap();
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

        None // Target not reached within max cycles
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::day_8::parser::{read_file, parse_cycle_instruction, parse_nodes};


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

        let cycle_instructions = parse_cycle_instruction(&file_content)
            .expect("Failed to parse cycle instructions");

        let nodes = parse_nodes(&file_content)
            .expect("Failed to parse nodes");

        let mut graph = Graph::new();
        for node in nodes {
            graph.add_node(node);
        }

        let start = "AAA";
        let target = "ZZZ";
        let max_cycles = 10;

        let traversal_result = graph.traverse(start, target, &cycle_instructions, max_cycles)
            .expect("Traversal failed");

        assert_eq!(traversal_result.1, expected_steps);
    }
}

