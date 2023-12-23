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
                let connections = match ch {
                    '|' => vec![(x, y - 1), (x, y + 1)],
                    '-' => vec![(x - 1, y), (x + 1, y)],
                    'L' => vec![(x, y - 1), (x + 1, y)],
                    'J' => vec![(x, y - 1), (x - 1, y)],
                    '7' => vec![(x, y + 1), (x - 1, y)],
                    'F' => vec![(x, y + 1), (x + 1, y)],
                    'S' => vec![],
                    '.' => vec![],
                    _ => vec![],
                };
                map_tiles.insert((x, y), Tile::new((x, y), connections));
            }
        }
        map_tiles
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
        env_logger::init();

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
    use super::*;
    use crate::utils::input_output::read_file;

    #[test]
    fn test_read_file() {
        let content = read_file("resources/input_day_10_test_a.txt").unwrap();
        assert!(!content.is_empty());
    }

    #[test]
    fn test_get_map_tile_data() {
        let content = read_file("resources/input_day_10_test_a.txt").unwrap();
        let explorer = Explorer::new(&content);
        let map =explorer.get_map_tile_data();
        let possible_connections = explorer.remove_tiles_without_receptacle(map);

        assert_eq!(possible_connections.len(),7)
    }

    #[test]
    fn test_build_loop() {
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

    #[test]
    fn test_find_furthest_distance() {
        let content = read_file("resources/input_day_10_test_a.txt").unwrap();
        let explorer = Explorer::new(&content);
        let map =explorer.get_map_tile_data();
        let mut possible_connections = explorer.remove_tiles_without_receptacle(map);
        let pipe_loop = explorer.find_connected_tiles(&mut possible_connections);
        let furthest_position = explorer.find_furthest_distance(&pipe_loop);
        assert_eq!(furthest_position,4);
    }
}