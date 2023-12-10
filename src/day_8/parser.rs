use crate::day_8::node::Node;

use crate::day_8::errors::WaypointInstructionError;

pub fn parse_waypoint_instructions(input: &str) -> Result<String, WaypointInstructionError> {
    let first_line = input.lines()
        .next()
        .map(str::trim)
        .ok_or(WaypointInstructionError::EmptyInput)?
        .to_string();

    validate_waypoint_instructions(&first_line)?;

    Ok(first_line)
}

fn validate_waypoint_instructions(instruction: &str) -> Result<(), WaypointInstructionError> {
    if instruction.chars().all(|c| c == 'R' || c == 'L') {
        Ok(())
    } else {
        Err(WaypointInstructionError::InvalidCharacter)
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
