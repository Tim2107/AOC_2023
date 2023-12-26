mod utils;
mod day_8;
mod day_9;
mod day_1;
mod day_10;
mod day_11;

fn main() {

    match day_1::solve_day_1() {
        Ok(part_1_sum) => {
            println!("Day 1, Part 1: The sum is {}", part_1_sum);
            //println!("Day 8, Part 2: {} steps", part_2_steps);
        },
        Err(err) => { eprintln!("Error solving Day 1: {}", err); }
    }

    println!();

    match day_8::solve_day_8() {
        Ok((part_1_steps, part_2_steps)) => {
            println!("Day 8, Part 1: {} steps", part_1_steps);
            println!("Day 8, Part 2: {} steps", part_2_steps);
        },
        Err(err) => { eprintln!("Error solving Day 8: {}", err); }
    }

    println!();

    match day_9::solve_day_9() {
        Ok((part_1_sum,part_2_sum)) => {
            println!("Day 9, Part 1: The sum of the extrapolated next numbers is {}", part_1_sum);
            println!("Day 9, Part 2: The sum of the extrapolated preceding numbers is {}", part_2_sum);
        },
        Err(err) => { eprintln!("Error solving Day 9: {}", err); }
    }

    println!();

    match day_10::solve_day_10() {
        Ok((furthest_distance, enclosed_tile_count)) => {
            println!("Day 10, Part 1: The furthest distance from the starting position is {} steps", furthest_distance);
            println!("Day 10, Part 1: There are {} enclosed tiles in the loop", enclosed_tile_count);
        },
        Err(err) => { eprintln!("Error solving Day 10: {}", err); }
    }
}
