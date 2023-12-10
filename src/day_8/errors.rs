#[derive(Debug)]
pub enum TraversalError {
    InvalidInstruction,
    NodeNotFound,
    CycleLimitReached(usize),
}
