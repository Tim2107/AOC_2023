use std::collections::HashMap;
use crate::day_10::tile::Tile;

pub struct LoopIterator<'a> {
    tile_loop: &'a HashMap<(usize, usize), Tile>,
    start_position: (usize, usize),
    current_position: (usize, usize),
    previous_position: (usize, usize),
    has_started: bool,
}

impl<'a> LoopIterator<'a> {
    pub fn new(connected_tiles: &'a HashMap<(usize, usize), Tile>, start_position: (usize, usize)) -> Self {
        Self {
            tile_loop: connected_tiles,
            start_position,
            current_position: start_position,
            previous_position: start_position,
            has_started: false,
        }
    }
}

impl<'a> Iterator for LoopIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_started && self.current_position == self.start_position {
            return None;
        }

        self.has_started = true;
        if let Some(tile) = self.tile_loop.get(&self.current_position) {
            if let Some(&next_position) = tile.connections().iter().find(|&&position| position != self.previous_position) {
                self.previous_position = self.current_position;
                self.current_position = next_position;
                Some(1)
            } else {
                None
            }
        } else {
            None
        }
    }
}
