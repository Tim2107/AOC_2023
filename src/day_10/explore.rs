use std::collections::HashMap;
use crate::day_10::parser::Parser;
use crate::day_10::tile::Tile;

pub struct Explorer {
    map: Vec<Vec<char>>,
    start_position: (usize, usize),
}

impl Explorer {
    pub fn new(input: &str) -> Self {
        let parsed_map = Parser::new(input);
        let start_position = Parser::find_starting_position(&parsed_map).unwrap();

        Explorer {
            map: parsed_map.map,
            start_position,
        }
    }

    pub fn get_map_tile_data(&self) -> HashMap<(usize, usize), Tile> {
        let mut map_tiles = HashMap::new();

        for (y, row) in self.map.iter().enumerate() {
            for (x, &ch) in row.iter().enumerate() {
                let mut connections = Vec::new();
                match ch {
                    '|' => {
                        if self.is_on_map(y as isize - 1) { connections.push((x, y - 1)); }
                        if self.is_on_map(x as isize + 1) { connections.push((x, y + 1)); }
                    },
                    '-' => {
                        if self.is_on_map(x as isize - 1) { connections.push((x - 1, y)); }
                        if self.is_on_map(x as isize + 1) { connections.push((x + 1, y)); }
                    },
                    'L' => {
                        if self.is_on_map(y as isize - 1) { connections.push((x, y - 1)); }
                        if self.is_on_map(x as isize + 1) { connections.push((x + 1, y)); }
                    },
                    'J' => {
                        if self.is_on_map(y as isize - 1) { connections.push((x, y - 1)); }
                        if self.is_on_map(x as isize - 1) { connections.push((x - 1, y)); }
                    },
                    '7' => {
                        if self.is_on_map(y as isize + 1) { connections.push((x, y + 1)); }
                        if self.is_on_map(x as isize - 1) { connections.push((x - 1, y)); }
                    },
                    'F' => {
                        if self.is_on_map(y as isize + 1) { connections.push((x, y + 1)); }
                        if self.is_on_map(x as isize + 1) { connections.push((x + 1, y)); }
                    },
                    _ => {},
                }
                map_tiles.insert((x, y), Tile::new((x, y), connections));
            }
        }
        map_tiles
    }

    fn is_on_map(&self, to_test: isize) -> bool {
        !(to_test < 0 || (self.map.len() as isize) < to_test)
    }

    pub fn remove_tiles_without_receptacle(&self, map_tile_data: HashMap<(usize, usize), Tile>) -> HashMap<(usize, usize), Tile> {
        map_tile_data.into_iter()
            .filter(|&(_, ref tile)| !tile.connections().is_empty())
            .collect()
    }

    pub fn find_furthest_distance(&self, connected_tiles: &HashMap<(usize, usize), Tile>) -> usize {
        let mut current_position = self.start_position;
        let mut previous_position = self.start_position;
        let mut total_jumps = 0;

        loop {
            if let Some(tile) = connected_tiles.get(&current_position) {
                if let Some(&next_position) = tile.connections().iter().find(|&&position| position != previous_position) {
                    total_jumps += 1;
                    previous_position = current_position;
                    current_position = next_position;
                } else {
                    break;
                }
            } else {
                break;
            }

            if current_position == self.start_position {
                break;
            }
        }

        total_jumps / 2
    }

    pub fn find_connected_tiles(&self, mut map_tile_data: &mut HashMap<(usize, usize), Tile>) -> HashMap<(usize, usize), Tile> {

        let mut connected_tiles = HashMap::new();
        let mut newly_added = vec![self.start_position];

        let start_tile_connections = self.get_start_tile_connections(map_tile_data);
        map_tile_data.insert(self.start_position, Tile::new(self.start_position, start_tile_connections));

        while !newly_added.is_empty() {
            let mut next_newly_added = Vec::new();

            for &position in &newly_added {
                if let Some(tile) = map_tile_data.get(&position) {
                    for &connection_data in tile.connections() {
                        if !connected_tiles.contains_key(&connection_data) && !newly_added.contains(&connection_data) {
                            connected_tiles.insert(connection_data, map_tile_data[&connection_data].clone());
                            next_newly_added.push(connection_data);
                        }
                    }
                }
            }

            newly_added = next_newly_added;
        }

        connected_tiles
    }

    pub fn get_start_tile_connections(&self, map_tile_data: &HashMap<(usize, usize), Tile>) -> Vec<(usize, usize)> {
        map_tile_data.iter()
            .filter(|&(_, tile)| tile.connections().contains(&self.start_position))
            .map(|(&pos, _)| pos)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;
    use crate::utils::input_output::read_file;

    #[test]
    fn test_is_on_map() {
        let content = read_file("resources/input_day_10_test_a.txt").unwrap();
        let explorer = Explorer::new(&content);
        assert!(explorer.is_on_map(4))
    }

    #[rstest]
    #[case("resources/input_day_10_test_a.txt",7)]
    #[case("resources/input_day_10_test_b.txt",24)]
    #[case("resources/input_day_10_test_c.txt",15)]
    #[case("resources/input_day_10_test_d.txt",22)]

    pub fn test_get_map_tile_data(#[case] input_file: &str, #[case] expected: usize) {

        let content = read_file(input_file).unwrap();
        let explorer = Explorer::new(&content);
        let map =explorer.get_map_tile_data();
        let tiles_with_receptacle = explorer.remove_tiles_without_receptacle(map);

        assert_eq!(tiles_with_receptacle.len(), expected)
    }

    #[test]
    fn test_print_loop() {
        let content = read_file("resources/input_day_10_test_a.txt").unwrap();
        let explorer = Explorer::new(&content);
        let map =explorer.get_map_tile_data();
        let mut possible_connections = explorer.remove_tiles_without_receptacle(map);
        let pipe_loop = explorer.find_connected_tiles(&mut possible_connections);
        for ((x, y), tile) in pipe_loop.iter() {
            println!("Tile at ({}, {}):", x, y);
            for connection in tile.connections() {
                println!("  Connected to: {:?}", connection);
            }
        }
    }

   #[rstest]
   #[case("resources/input_day_10_test_a.txt",4)]
   #[case("resources/input_day_10_test_b.txt",4)]
   #[case("resources/input_day_10_test_c.txt",8)]
   #[case("resources/input_day_10_test_d.txt",8)]

    fn test_find_furthest_distance(#[case] input_file:&str, #[case] furthest_distance:usize) {
        let content = read_file(input_file).unwrap();
        let explorer = Explorer::new(&content);
        let map =explorer.get_map_tile_data();
        let mut possible_connections = explorer.remove_tiles_without_receptacle(map);
        let pipe_loop = explorer.find_connected_tiles(&mut possible_connections);
        let furthest_position = explorer.find_furthest_distance(&pipe_loop);

        assert_eq!(furthest_position,furthest_distance);
    }
}