#[derive(Clone)]
pub struct Tile {
    tile_type: char,
    connections: Vec<(usize, usize)>,
    receptors: Vec<(isize, isize)>,
    lateral_data: Vec<Vec<(isize, isize)>>
}

impl Tile {
    pub fn new(tile_type: char, connections: Vec<(usize, usize)>) -> Self {
        let receptors = match tile_type {
            '|' => vec![(0, -1), (0, 1)],
            '-' => vec![(-1, 0), (1, 0)],
            'L' => vec![(0, -1), (1, 0)],
            'J' => vec![(0, -1), (-1, 0)],
            '7' => vec![(0, 1), (-1, 0)],
            'F' => vec![(0, 1), (1, 0)],
            _ => vec![]
        };

        let lateral_data = match tile_type {
            '|' => {
                let lateral_forward_right = vec![(-1, 0)];
                let lateral_forward_left = vec![(1, 0)];
                vec![lateral_forward_right, lateral_forward_left]
            },
            '-' => {
                let lateral_forward_right = vec![(0, 1)];
                let lateral_forward_left = vec![(0, -1)];
                vec![lateral_forward_right, lateral_forward_left]
            },
            'L' => {
                let lateral_forward_right = vec![(-1, 0), (0, 1)];
                let lateral_forward_left = vec![(1, 0), (0, -1)];
                vec![lateral_forward_right, lateral_forward_left]
            },
            'J' => {
                let lateral_forward_right = vec![(-1, 0), (0, -1)];
                let lateral_forward_left = vec![(1, 0), (0, 1)];
                vec![lateral_forward_right, lateral_forward_left]
            },
            '7' => {
                let lateral_forward_right = vec![(1, 0), (0, -1)];
                let lateral_forward_left = vec![(-1, 0), (0, 1)];
                vec![lateral_forward_right, lateral_forward_left]
            },
            'F' => {
                let lateral_forward_right = vec![(1, 0), (0, 1)];
                let lateral_forward_left = vec![(-1, 0), (0, -1)];
                vec![lateral_forward_right, lateral_forward_left]
            },
            _ => vec![]
        };

        Tile {
            tile_type,
            connections,
            receptors,
            lateral_data,
        }
    }
    pub fn set_connections(&mut self, connections: Vec<(usize, usize)>) {
        self.connections = connections;
    }
    pub fn connections(&self) -> &Vec<(usize, usize)> {
        &self.connections
    }
    pub fn tile_type(&self) -> &char { &self.tile_type }
    pub fn receptors(&self) -> &Vec<(isize,isize)> { &self.receptors }
    pub fn lateral_data(&self) -> &Vec<Vec<(isize,isize)>> { &self.lateral_data }
}
