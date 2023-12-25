use std::collections::HashMap;
use crate::day_10::parser::Parser;
use crate::day_10::tile::Tile;
use crate::day_10::loop_iterator::LoopIterator;

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

    pub fn find_furthest_distance(&self) -> usize {
        let map_data = self.get_map_tile_data();
        let tile_loop_iterator = LoopIterator::new(&map_data, self.start_position);
        let total_jumps = tile_loop_iterator.map(|(count, _, _)| count).sum::<usize>();
        total_jumps / 2
    }

    pub fn count_enclosed_tiles(&self) {
        let map_data = self.get_map_tile_data();
        let tile_loop_iterator = LoopIterator::new(&map_data, self.start_position);

        for (_, position,is_forward) in tile_loop_iterator {
            let tile_char = self.map[position.0][position.1];
            println!("Tile: {}, is_forward: {}", tile_char, is_forward);
        }
    }

    pub fn get_map_tile_data(&self) -> HashMap<(usize, usize), Tile> {
        let mut tile_data_map = HashMap::new();

        for (x, y, tile_char) in self.iterate_map_tiles() {
            let tile = self.process_tile_data(x, y, tile_char);
            tile_data_map.insert((x, y), tile);
        }

        let start_tile_connections = self.get_start_tile_connections(&tile_data_map);
        tile_data_map.insert(self.start_position, Tile::new('S',start_tile_connections));

        tile_data_map
    }

    fn iterate_map_tiles<'a>(&'a self) -> impl Iterator<Item = (usize, usize, char)> + 'a {
        self.map.iter().enumerate().flat_map(move |(y, row)| {
            row.iter().enumerate().map(move |(x, &tile_char)| (x, y, tile_char))
        })
    }

    pub fn process_tile_data(&self, x: usize, y: usize, tile_char: char) -> Tile {
        let mut tile = Tile::new(tile_char, vec![],);
        let receptors = tile.receptors();

        let mut updated_connections = Vec::new();
        for (dx, dy) in receptors {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;

            if self.is_on_map(new_x, new_y) {
                updated_connections.push((new_x as usize, new_y as usize));
            }
        }

        tile.set_connections(updated_connections);
        tile
    }

    fn is_on_map(&self, x: isize, y: isize) -> bool {
        x >= 0 && (x as usize) < self.map[0].len() && y >= 0 && (y as usize) < self.map.len()
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
        assert!(explorer.is_on_map(3,2))
    }

    #[rstest]
    #[case("resources/input_day_10_test_a.txt",8)]
    #[case("resources/input_day_10_test_b.txt",24)]
    #[case("resources/input_day_10_test_c.txt",16)]
    #[case("resources/input_day_10_test_d.txt",23)]

    pub fn test_get_map_tile_data(#[case] input_file: &str, #[case] expected: usize) {

        let content = read_file(input_file).unwrap();
        let explorer = Explorer::new(&content);
        let map =explorer.get_map_tile_data();
        let tiles_with_receptacle = remove_tiles_without_receptacle(map);

        assert_eq!(tiles_with_receptacle.len(), expected)
    }

    pub fn remove_tiles_without_receptacle(map_tile_data: HashMap<(usize, usize), Tile>) -> HashMap<(usize, usize), Tile> {
        map_tile_data.into_iter()
            .filter(|&(_, ref tile)| !tile.connections().is_empty())
            .collect()
    }

   #[rstest]
   #[case("resources/input_day_10_test_a.txt",4)]
   #[case("resources/input_day_10_test_b.txt",4)]
   #[case("resources/input_day_10_test_c.txt",8)]
   #[case("resources/input_day_10_test_d.txt",8)]

    fn test_find_furthest_distance(#[case] input_file:&str, #[case] furthest_distance:usize) {
        let content = read_file(input_file).unwrap();
        let explorer = Explorer::new(&content);
        let furthest_position = explorer.find_furthest_distance();

        assert_eq!(furthest_position,furthest_distance);
    }

    #[rstest]
    #[case("resources/input_day_10_test_a.txt")]
    fn test_visualize_lateral_data(#[case] input_file:&str){
        let content = read_file(input_file).unwrap();
        let explorer = Explorer::new(&content);
        let furthest_position = explorer.count_enclosed_tiles();
    }
}