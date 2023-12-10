#[derive(Debug)]
pub enum GraphError {
    NodeNotFound(String),
    TargetNotReachedWithinIterations,
    NeighbourNodeNotFound { node: String, neighbour: String },
}

impl std::fmt::Display for GraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GraphError::NodeNotFound(node) => write!(f, "Node not found: {}", node),
            GraphError::TargetNotReachedWithinIterations => write!(f, "Target not reached within maximum waypoint instruction iterations"),
            GraphError::NeighbourNodeNotFound { node, neighbour } => write!(f, "Neighbour node '{}' not found from '{}'", neighbour, node),
        }
    }
}

impl std::error::Error for GraphError {}

#[derive(Debug)]
pub enum Day8ParsingError{
    EmptyInput,
    InvalidWaypointInstruction,
    InvalidNodeDefinition,
    InvalidConnectionCount,
}

impl  std::fmt::Display for Day8ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Day8ParsingError::EmptyInput => write!(f, "Input file is empty"),
            Day8ParsingError::InvalidWaypointInstruction => write!(f, "Waypoints contain invalid characters"),
            Day8ParsingError::InvalidNodeDefinition => write!(f, "Invalid node definition"),
            Day8ParsingError::InvalidConnectionCount => write!(f, "Invalid number of connections"),
        }
    }
}

impl std::error::Error for Day8ParsingError{}

