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

    pub fn get_receptors(tile_char: char) -> Vec<(isize, isize)> {
        match tile_char {
            '|' => vec![(  0 , -1 ) , (  0 , 1 )],
            '-' => vec![( -1 ,  0 ) , (  1 , 0 )],
            'L' => vec![(  0 , -1 ) , (  1 , 0 )],
            'J' => vec![(  0 , -1 ) , ( -1 , 0 )],
            '7' => vec![(  0 ,  1 ) , ( -1 , 0 )],
            'F' => vec![(  0 ,  1 ) , (  1 , 0 )],
            _ => vec![]
        }
    }
}
