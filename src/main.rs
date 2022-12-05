mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use chrono::Duration;

mod input;
use day1::run_part_1;
use input::get_input;
fn main() {
    let input = get_input(5);
    let start = std::time::Instant::now();
    println!();
    println!("Test: \n{} " ,day5::solve_part_2(&input));
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    
}


