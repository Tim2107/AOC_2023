use std::collections::{HashMap, HashSet};
use crate::utils::math::{least_common_multiple};
use crate::day_8::parser::{parse_waypoint_instructions, parse_nodes};
use crate::day_8::node::Node;

#[derive(Debug)]
pub enum TraversalError {
    InvalidInstruction,
    NodeNotFound,
    CycleLimitReached(usize),
}

pub struct Graph {
    nodes: HashMap<String, Node>,
    waypoint_instructions: String,
}

impl Graph {
    pub fn new(input: &str) -> Result<Graph, String> {
        let waypoint_instructions = parse_waypoint_instructions(input)?;
        let parsed_nodes = parse_nodes(input)?;
        let mut nodes = HashMap::new();
        for node in parsed_nodes {
            nodes.insert(node.name().to_string(), node);
        }

        Ok(Graph {
            nodes,
            waypoint_instructions,
        })
    }

    pub fn traverse(&self, start: &str, target: &str, max_cycles: usize) -> Option<(String, usize)> {
        let mut current_node = start;
        let mut current_cycle = 0;
        let mut steps = 0;
        let instruction_len = self.waypoint_instructions.chars().count();

        while current_cycle < max_cycles {
            for i in 0..instruction_len {
                if let Some(node) = self.nodes.get(current_node) {
                    if current_node == target {
                        return Some((target.to_string(), steps));
                    }

                    let next_instruction = self.waypoint_instructions.chars().nth(i).unwrap();
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

    pub fn find_overall_step(&self) -> usize {
        let distances = self.find_target_distances();
        let mut cycle_lengths: Vec<usize> = Vec::new();

        for (_, target_distances) in distances {
            if let Some((_, first_distance)) = target_distances.get(0) {
                if let Some((_, second_distance)) = target_distances.get(1) {
                    if(first_distance==second_distance){
                        cycle_lengths.push(*first_distance);
                    } else {
                        cycle_lengths.push(*first_distance + *second_distance)
                    }
                }
            }
        }

        cycle_lengths.into_iter().reduce(|a, b| least_common_multiple(a, b)).unwrap_or(0)
    }


    pub fn find_target_distances(&self) -> HashMap<String, Vec<(String, usize)>> {
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
                let instruction = self.waypoint_instructions.chars().nth(total_steps % self.waypoint_instructions.len()).unwrap();
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
    use crate::day_8::parser::{read_file};

    #[test]
    fn testcase_a(){
        test_add_and_traverse_graph_helper("resources/input_day_8_test_a.txt",2)
    }

    #[test]
    fn testcase_b(){
        test_add_and_traverse_graph_helper("resources/input_day_8_test_b.txt",6)
    }

    fn test_add_and_traverse_graph_helper(testfile_path: &str, expected_steps: usize ) {

        let mut graph = Graph::new(&read_file(testfile_path).unwrap());

        let start = "AAA";
        let target = "ZZZ";
        let max_cycles = 10;

        let traversal_result = graph.traverse(start, target, max_cycles)
            .expect("Traversal failed");

        assert_eq!(traversal_result.1, expected_steps);
    }

    #[test]
    fn test_find_target_distances() {

        let mut graph = Graph::new(&read_file("resources/input_day_8_test_c.txt").unwrap());
        let distances = graph.find_target_distances();

        for (start_node, target_distances) in distances {
            println!("Start Node: {}", start_node);
            for (target, distance) in target_distances {
                println!(" -> Distance to '{}': {} steps", target, distance);
            }
        }
    }

    #[test]
    fn test_find_overall_step() {
        let mut graph = Graph::new(&read_file("resources/input_day_8_test_c.txt").unwrap());
        let overall_step = graph.find_overall_step();
        assert_eq!(overall_step,6);
    }
}

