use std::collections::{HashMap, HashSet};
use crate::utils::math::{least_common_multiple};
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

    pub fn find_overall_step(&self, waypoint_instructions: &str) -> usize {
        let distances = self.find_target_distances(waypoint_instructions);
        let mut cycle_lengths: Vec<usize> = Vec::new();

        for (_, target_distances) in distances {
            if let Some((_, first_distance)) = target_distances.get(0) {
                if let Some((_, second_distance)) = target_distances.get(1) {
                    cycle_lengths.push(*first_distance);
                }
            }
        }

        cycle_lengths.into_iter().reduce(|a, b| least_common_multiple(a, b)).unwrap_or(0)
    }


    pub fn find_target_distances(&self, waypoint_instructions: &str) -> HashMap<String, Vec<(String, usize)>> {
        let mut distances = HashMap::new();
        let start_nodes: Vec<_> = self.nodes.values()
            .filter(|node| node.is_start_node())
            .collect();

        for node in start_nodes {
            let mut visited_targets = HashSet::new();
            let mut current_node = node.name();
            let mut steps_since_last_target = 0;
            let mut total_steps = 0;

            loop {
                let instruction = waypoint_instructions.chars().nth(total_steps % waypoint_instructions.len()).unwrap();
                current_node = match instruction {
                    'R' => &self.nodes[current_node].right_neighbour(),
                    'L' => &self.nodes[current_node].left_neighbour(),
                    _ => panic!("Invalid waypoint instruction"),
                };
                total_steps += 1;
                steps_since_last_target += 1;

                if current_node.ends_with('Z') {
                    if visited_targets.contains(current_node) {
                        distances.entry(node.name().to_string())
                            .or_insert_with(Vec::new)
                            .push((current_node.to_string(), steps_since_last_target));
                        break;
                    }
                    visited_targets.insert(current_node.to_string());
                    distances.entry(node.name().to_string())
                        .or_insert_with(Vec::new)
                        .push((current_node.to_string(), steps_since_last_target));
                    steps_since_last_target = 0;
                }
            }
        }

        distances
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
    fn test_find_target_distances() {
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
        let distances = graph.find_target_distances(&waypoint_instructions);

        for (start_node, target_distances) in distances {
            println!("Start Node: {}", start_node);
            for (target, distance) in target_distances {
                println!(" -> Distance to '{}': {} steps", target, distance);
            }
        }
    }

    #[test]
    fn test_find_overall_step() {
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
        let overall_step = graph.find_overall_step(&waypoint_instructions);
        assert_eq!(overall_step,6);
    }
}

