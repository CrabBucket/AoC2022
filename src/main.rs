mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use chrono::Duration;

mod input;
use day1::run_part_1;
use input::get_input;
fn main() {
    let input = get_input(9);
    let start = std::time::Instant::now();
    println!();
    println!("Test: \n{} " ,day9::solve_part_2(input.to_string()));
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    
}


