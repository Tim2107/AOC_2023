#[derive(Debug, PartialEq)]
pub struct Node {
    name: String,
    left_neighbour: String,
    right_neighbour: String,
}

impl Node {
    pub fn new(name: &str, left_neighbour: &str, right_neighbour: &str) -> Self {
        Node {
            name: name.to_string(),
            left_neighbour: left_neighbour.to_string(),
            right_neighbour: right_neighbour.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn left_neighbour(&self) -> &str {
        &self.left_neighbour
    }

    pub fn right_neighbour(&self) -> &str {
        &self.right_neighbour
    }

    pub fn is_start_node(&self) -> bool {
        self.name.ends_with('A')
    }

    pub fn is_target_node(&self) -> bool {
        self.name.ends_with('Z')
    }
}
