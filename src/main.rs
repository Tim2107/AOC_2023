mod utils;
mod day_8;

fn main() {

    let solution_day_8_part_1 = day_8::solve_day_8_part_1();
    let solution_day_8_part_2 = day_8::solve_day_8_part_2();

    println!("The solution to day 8 - part 1 is {} steps.", solution_day_8_part_1);
    println!("The solution to day 8 - part 2 is {} steps.", solution_day_8_part_2);
}
