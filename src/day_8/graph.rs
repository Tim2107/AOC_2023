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

    pub fn traverse(&self, start: &str, target: &str, max_cycles: usize) -> Result<usize, String> {
        let mut current_node = start;
        let mut current_cycle = 0;
        let mut steps = 0;
        let instruction_len = self.waypoint_instructions.chars().count();

        while current_cycle < max_cycles {
            for i in 0..instruction_len {
                let next_node = self.get_next_node(current_node, self.waypoint_instructions.chars().nth(i).unwrap())?;
                current_node = next_node.name();

                if current_node == target {
                    return Ok(steps);
                }
                steps += 1;
            }
            current_cycle += 1;
        }

        Err("Target not reached within maximum cycles".to_string())
    }

    pub fn find_overall_step(&self) ->  Result<usize, String> {
        let distances = self.find_target_distances()?;
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

        Ok(cycle_lengths.into_iter().reduce(|a, b| least_common_multiple(a, b)).unwrap_or(0))
    }


    pub fn find_target_distances(&self) -> Result<HashMap<String, Vec<(String, usize)>>, String> {
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
                let next_node = self.get_next_node(current_node, instruction)?;
                current_node = next_node.name();
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

        Ok(distances)
    }

    fn get_next_node(&self, current_node: &str, instruction: char) -> Result<&Node, String> {
        let node = self.nodes.get(current_node)
            .ok_or_else(|| "Current node not found in the graph".to_string())?;

        match instruction {
            'R' => self.nodes.get(node.right_neighbour())
                .ok_or_else(|| "Right neighbor not found".to_string()),
            'L' => self.nodes.get(node.left_neighbour())
                .ok_or_else(|| "Left neighbor not found".to_string()),
            _ => Err("Cannot get next node -> waypoint instruction is invalid.".to_string()),
        }
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

