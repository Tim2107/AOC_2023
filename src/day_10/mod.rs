use crate::day_10::explore::Explorer;
use crate::utils::input_output::read_file;
use anyhow::{Result};

mod parser;
mod explore;
mod tile;
mod loop_iterator;

pub fn solve_day_10() -> Result<(usize,usize)> {
        let content = read_file("resources/input_day_10.txt")?;
        let explorer = Explorer::new(&content);
        let furthest_position = explorer.find_furthest_distance();
        let enclosed_tile_count = explorer.count_enclosed_tiles();
        Ok((furthest_position, enclosed_tile_count))
}