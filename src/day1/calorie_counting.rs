use std::thread::current;

use crate::input::get_input;

use super::super::input;

pub fn run() {
    let input = get_input();
    let split = input.split("\n\n");
    let mut big_elf = -1;
    for elf in split {
        let mut current_elf = 0;
        let elf_split = elf.split('\n');
        for calories in elf_split{
            current_elf+= match calories.parse::<i32>() {
                Ok(n) => n,
                Err(_) => 0,
            };
        }
        if current_elf > big_elf {
            big_elf = current_elf;
        }
    }
    println!("Biggest Elf: {}",big_elf);

}

