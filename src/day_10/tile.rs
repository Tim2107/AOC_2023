#[derive(Clone)]
pub struct Tile {
    position: (usize, usize),
    pub connections: Vec<(usize, usize)>,
}

impl Tile {
    pub fn new(position: (usize, usize), connections: Vec<(usize, usize)>) -> Self {
        Tile { position, connections }
    }

    pub fn position(&self) -> (usize, usize) {
        self.position
    }

    pub fn connections(&self) -> &Vec<(usize, usize)> {
        &self.connections
    }

    pub fn set_connections(&mut self, connections: Vec<(usize, usize)>) {
        self.connections = connections;
    }
}
