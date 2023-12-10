mod utils;
mod day_8;

fn main() {

    match day_8::solve_day_8() {
        Ok((part_1_steps, part_2_steps)) => {
            println!("Day 8, Part 1: {} steps", part_1_steps);
            println!("Day 8, Part 2: {} steps", part_2_steps);
        },
        Err(err) => { eprintln!("Error solving Day 8: {}", err); }
    }
}
