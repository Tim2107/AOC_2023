use std::fs;
use std::io;
use std::path::Path;

use crate::day_8::node::Node;

pub fn read_file<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
    fs::read_to_string(file_path)
}

pub fn parse_cycle_instruction(input: &str) -> Result<String, &'static str> {
    let first_line = input.lines().next().ok_or("Input file is empty")?;

    if first_line.trim().chars().all(|c| c == 'R' || c == 'L') {
        Ok(first_line.trim().to_string())
    } else {
        Err("Cycle instruction contains invalid characters")
    }
}

pub fn parse_nodes(input: &str) -> Result<Vec<Node>, &'static str> {
    input.lines()
        .filter(|line| !line.is_empty() && line.contains('='))
        .map(|line| {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() != 2 {
                return Err("Invalid node definition");
            }

            let name = parts[0].trim();
            let connections: Vec<&str> = parts[1].trim().trim_matches(|p| p == '(' || p == ')').split(',').map(str::trim).collect();
            if connections.len() != 2 {
                return Err("Invalid number of connections");
            }

            Ok(Node::new(name, connections[0], connections[1]))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let content = read_file("resources/input_day_8_test_a.txt").unwrap();
        assert!(!content.is_empty());
    }

    #[test]
    fn test_parse_cycle_instruction() {
        let content = read_file("resources/input_day_8_test_a.txt").unwrap();
        let cycle_instruction = "RL";
        assert_eq!(parse_cycle_instruction(&content).unwrap(), cycle_instruction);
    }

    #[test]
    fn test_parse_nodes() {
        let file_content = read_file("resources/input_day_8_test_a.txt")
            .expect("Failed to read test file");

        let nodes = parse_nodes(&file_content)
            .expect("Failed to parse nodes");

        let expected_node = Node::new("AAA", "BBB", "CCC");
        let unexpected_node = Node::new("AAA","BBB", "KGB");

        assert!(nodes.contains(&expected_node));
        assert!(!nodes.iter().any(|node| *node == unexpected_node));
    }
}
