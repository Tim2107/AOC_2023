use std::collections::HashMap;
use crate::day_10::tile::Tile;
use crate::utils::collections::collection_operations::{add_tuples, subtract_tuples};

pub struct LoopIterator<'a> {
    tile_map: &'a HashMap<(usize, usize), Tile>,
    start_position: (usize, usize),
    current_position: (usize, usize),
    previous_position: (usize, usize),
    has_started: bool,
}

impl<'a> LoopIterator<'a> {
    pub fn new(connected_tiles: &'a HashMap<(usize, usize), Tile>, start_position: (usize, usize), start_direction_exclusion: (usize, usize)) -> Self {
        Self {
            tile_map: connected_tiles,
            start_position,
            current_position: start_position,
            previous_position: start_direction_exclusion,
            has_started: false,
        }
    }

    fn determine_if_forward(&self, previous_position: (usize, usize), current_position: (usize, usize), tile_data: &Tile) -> bool {

        let entry_point = subtract_tuples (previous_position, current_position);

        if tile_data.receptors().contains(&entry_point){
            if tile_data.receptors()[0] == entry_point {
                return true;
            }
        }
        false
    }

    fn positions_to_check_right(&self, current_position:(usize, usize), inspected_tile: &Tile, is_forward: bool) -> Vec<(usize, usize)> {
        let mut to_inspect = Vec::new();

        if !(&inspected_tile.lateral_data().is_empty()){
            if is_forward {
                for positions_to_check in &inspected_tile.lateral_data()[0]{
                    to_inspect.push(add_tuples(current_position, *positions_to_check))
                }
            } else {
                for positions_to_check in &inspected_tile.lateral_data()[1]{
                    to_inspect.push(add_tuples(current_position, *positions_to_check))
                }
            }
        }

        let to_inspect: Vec<(usize, usize)> = to_inspect.iter()
            .map(|&(x, y)| (x.max(0) as usize, y.max(0) as usize))
            .collect();

        to_inspect
    }
}

impl<'a> Iterator for LoopIterator<'a> {
    type Item = (usize,(usize, usize), Vec<(usize, usize)>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_started && self.current_position == self.start_position {
            return None;
        }

        self.has_started = true;
        if let Some(tile) = self.tile_map.get(&self.current_position) {
            if let Some(&next_position) = tile.connections().iter().find(|&&position| position != self.previous_position) {
                let is_forward = self.determine_if_forward(self.previous_position, self.current_position, tile);

                self.previous_position = self.current_position;
                self.current_position = next_position;
                let positions_to_check =  self.positions_to_check_right(self.previous_position, &tile, is_forward);

                Some((1, self.previous_position , positions_to_check))
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests{
    use crate::day_10::explore::Explorer;
    use crate::utils::input_output::read_file;

    #[test]
    fn test_visualize_lateral_data1(){
        let content = read_file("resources/input_day_10_test_c.txt").unwrap();
        let explorer = Explorer::new(&content);
        explorer.count_enclosed_tiles();
    }
}