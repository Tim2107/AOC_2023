use crate::day_10::explore::Explorer;
use crate::utils::input_output::read_file;
use anyhow::{Result};

mod parser;
mod explore;
mod tile;

pub fn solve_day_10() -> Result<usize> {
        let content = read_file("resources/input_day_10.txt")?;
        let explorer = Explorer::new(&content);
        let map =explorer.get_map_tile_data();
        let mut possible_connections = explorer.remove_tiles_without_receptacle(map);
        let pipe_loop = explorer.get_loop(&mut possible_connections);
        let furthest_position = explorer.find_furthest_distance(&pipe_loop);
        Ok(furthest_position)
}