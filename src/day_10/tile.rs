#[derive(Clone)]
pub struct Tile {
    pub connections: Vec<(usize, usize)>,
}

impl Tile {
    pub fn new(connections: Vec<(usize, usize)>) -> Self {
        Tile { connections }
    }

    pub fn connections(&self) -> &Vec<(usize, usize)> {
        &self.connections
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
