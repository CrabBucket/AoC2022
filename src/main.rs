#![allow(dead_code)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod day12;





mod input;
use input::get_input;
fn main() {
    let input = get_input(12);
    let start = std::time::Instant::now();
    println!();
    println!("Test: \n{} " ,day12::solve_part_2(std::fs::read_to_string("C:\\Users\\19513\\Desktop\\It Projects Local\\AoC2022\\src\\input").unwrap()));
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    
}


