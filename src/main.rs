mod utils;
mod day_8;
mod day_9;

fn main() {

    match day_8::solve_day_8() {
        Ok((part_1_steps, part_2_steps)) => {
            println!("Day 8, Part 1: {} steps", part_1_steps);
            println!("Day 8, Part 2: {} steps", part_2_steps);
        },
        Err(err) => { eprintln!("Error solving Day 8: {}", err); }
    }

    println!();

    match day_9::solve_day_9() {
        Ok(part_q1_sum) => {
            println!("Day 9, Part 1: The sum of the extrapolated numbers is {}", part_q1_sum);
        },
        Err(err) => { eprintln!("Error solving Day 9: {}", err); }
    }
}
