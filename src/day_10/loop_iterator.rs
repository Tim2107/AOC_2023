use std::collections::HashMap;
use crate::day_10::tile::Tile;
use crate::utils::collection_operations::subtract_tuples;

pub struct LoopIterator<'a> {
    tile_map: &'a HashMap<(usize, usize), Tile>,
    start_position: (usize, usize),
    current_position: (usize, usize),
    previous_position: (usize, usize),
    has_started: bool,
}

impl<'a> LoopIterator<'a> {
    pub fn new(connected_tiles: &'a HashMap<(usize, usize), Tile>, start_position: (usize, usize)) -> Self {
        Self {
            tile_map: connected_tiles,
            start_position,
            current_position: start_position,
            previous_position: start_position,
            has_started: false,
        }
    }

    fn determine_if_forward(&self, prev_position: (usize, usize), curr_position: (usize, usize), tile_data: &Tile) -> bool {

        let entry_point = subtract_tuples (curr_position, prev_position);

        if tile_data.receptors().contains(&entry_point){
            if tile_data.receptors()[0] == entry_point {
                return true;
            }
        }
        false
    }
}

impl<'a> Iterator for LoopIterator<'a> {
    type Item = (usize,(usize, usize) ,bool);

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

                Some((1, self.previous_position ,is_forward))
            } else {
                None
            }
        } else {
            None
        }
    }
}

