#[derive(Debug)]
pub enum TraversalError {
    InvalidInstruction,
    NodeNotFound,
    CycleLimitReached(usize),
}

#[derive(Debug)]
pub enum WaypointInstructionError {
    EmptyInput,
    InvalidCharacter,
}

impl std::fmt::Display for WaypointInstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WaypointInstructionError::EmptyInput => write!(f, "Input file is empty"),
            WaypointInstructionError::InvalidCharacter => write!(f, "Waypoints contain invalid characters"),
        }
    }
}

impl std::error::Error for WaypointInstructionError {}

