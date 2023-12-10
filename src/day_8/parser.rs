use crate::day_8::node::Node;
use crate::day_8::errors::{Day8ParsingError};

pub fn parse_waypoint_instructions(input: &str) -> Result<String, Day8ParsingError> {
    let first_line = input.lines()
        .next()
        .map(str::trim)
        .ok_or(Day8ParsingError::EmptyInput)?
        .to_string();

    validate_waypoint_instructions(&first_line)?;

    Ok(first_line)
}

fn validate_waypoint_instructions(instruction: &str) -> Result<(), Day8ParsingError> {
    if instruction.chars().all(|c| c == 'R' || c == 'L') {
        Ok(())
    } else {
        Err(Day8ParsingError::InvalidWaypointInstruction)
    }
}

pub fn parse_nodes(input: &str) -> Result<Vec<Node>, Day8ParsingError> {
    input.lines()
        .filter(|line| !line.is_empty() && line.contains('='))
        .map(|line| {
            let parts: Vec<&str> = line.split('=').collect();
            validate_node_definition(&parts)?;

            let name = parts[0].trim();
            let connections: Vec<&str> = parts[1]
                    .trim()
                    .trim_matches(|p| p == '(' || p == ')')
                    .split(',').map(str::trim)
                    .collect();
            validate_node_connections(&connections)?;

            Ok(Node::new(name, connections[0], connections[1]))
        })
        .collect()
}

fn validate_node_definition(parts: &[&str]) -> Result<(), Day8ParsingError> {
    if parts.len() != 2 {
        Err(Day8ParsingError::InvalidNodeDefinition)
    } else {
        Ok(())
    }
}

fn validate_node_connections(connections: &[&str]) -> Result<(), Day8ParsingError> {
    if connections.len() != 2 {
        Err(Day8ParsingError::InvalidConnectionCount)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::input_output::read_file;

    #[test]
    fn test_read_file() {
        let content = read_file("resources/input_day_8_test_a.txt").unwrap();
        assert!(!content.is_empty());
    }

    #[test]
    fn test_parse_waypoint_instructions() {
        let content = read_file("resources/input_day_8_test_a.txt").unwrap();
        assert_eq!(parse_waypoint_instructions(&content).unwrap(), "RL");
    }

    #[test]
    fn test_parse_nodes() {
        let file_content = read_file("resources/input_day_8_test_a.txt").unwrap();
        let nodes = parse_nodes(&file_content).unwrap();

        let expected_node = Node::new("AAA", "BBB", "CCC");
        let unexpected_node = Node::new("AAA","BBB", "KGB");

        assert!(nodes.contains(&expected_node));
        assert!(!nodes.iter().any(|node| *node == unexpected_node));
    }
}
