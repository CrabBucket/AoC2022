use std::collections::HashMap;

pub fn solve_part_1(input: &str) {
    let fixed_input = input.trim_end();
    let lines = fixed_input.lines();
    let ave = &fixed_input.lines().count() * 26;
    let mut first_bin: HashMap<char, bool> = HashMap::new();
    let mut second_bin: HashMap<char, bool> = HashMap::new();
    let mut shared_chars = vec![];
    for line in lines {
        let compartments = line.split_at(line.len() / 2);
        // println!("{} : {}", compartments.0, compartments.1);
        let (mut first, mut second) = (compartments.0.chars(), compartments.1.chars());
        // if line == "fggNNffGmcBrmBfcDzzzpHbsGTpszwwTbp" {
        //     println!("{} : {}", compartments.0, compartments.1);
        // }
        for index in 0..compartments.0.len() {
            let first_char = first.nth(0).unwrap();
            let second_char = second.nth(0).unwrap();

            if second_bin.contains_key(&first_char) {
                shared_chars.push(first_char);
                break;
            }
            if first_bin.contains_key(&second_char) {
                shared_chars.push(second_char);
                break;
            }
            if first_char == second_char {
                shared_chars.push(first_char);
                break;
            }
            first_bin.insert(first_char, true);
            second_bin.insert(second_char, true);
        }
        first_bin.clear();
        second_bin.clear();
    }
    // let mut sum = 0;
    // for char in &shared_chars {
    //     println!("{} the Value of this char is: {}", char, match char.is_ascii_lowercase() {
    //         true => *char as u8 - 96,
    //         false => *char as u8 - 38,
    //     });
    //     sum += match char.is_ascii_lowercase() {
    //         true => *char as u8 - 96,
    //         false => *char as u8 - 38,
    //     };
    // }
    let result = shared_chars
        .iter()
        .map(|c| match c.is_ascii_lowercase() {
            true => *c as u16 - 96,
            false => *c as u16 - 38,
        })
        .sum::<u16>();
    println!("The total sum is {}", result);
    // println!("The total sum is {}", sum);
    // println!("Average should be {}", ave/26);
    // println!("Should be {}", shared_chars.len());
}

pub fn solve_part_2(input: &str) {
    let fixed_input = input.trim_end();
    let lines = fixed_input.lines();
    let mut line_vec: Vec<&str> = lines.collect();
    let mut first_bin: HashMap<char, (bool, bool, bool)> = HashMap::new();
    let mut shared_chars = vec![];
    while !line_vec.is_empty() {
        let lines: Vec<&str> = line_vec.drain(0..3).collect();
        let (mut chars1, mut chars2, mut chars3) = (lines[0].chars(), lines[1].chars(), lines[2].chars());
        for index in 0..lines[0].len() {
            let char = chars1.nth(0).unwrap();
            if first_bin.contains_key(&char) {
                let flags = first_bin.get(&char).unwrap();
                first_bin.insert(char, (true, flags.1, flags.2));
            } else {
                first_bin.insert(char, (true, false, false));
            }
        }
        for index in 0..lines[1].len() {
            let char = chars2.nth(0).unwrap();
            if first_bin.contains_key(&char) {
                let flags = first_bin.get(&char).unwrap();
                first_bin.insert(char, (flags.0, true, flags.2));
            } else {
                first_bin.insert(char, (false, true, false));
            }
        }
        for index in 0..lines[2].len() {
            let char = chars3.nth(0).unwrap();
            if first_bin.contains_key(&char) {
                let flags = first_bin.get(&char).unwrap();
                first_bin.insert(char, (flags.0, flags.1, true));
            } else {
                first_bin.insert(char, (false, false, true));
            }
        }

        for (key, value) in &first_bin {
            if value == &(true, true, true) {
                shared_chars.push(*key);
            }
        }
        first_bin.clear();
    }

    let mut sum = 0;
    // for char in &shared_chars {
    //     println!(
    //         "{} the Value of this char is: {}",
    //         char,
    //         match char.is_ascii_lowercase() {
    //             true => *char as u16 - 96,
    //             false => *char as u16 - 38,
    //         }
    //     );
    //     sum += match char.is_ascii_lowercase() {
    //         true => *char as u16 - 96,
    //         false => *char as u16 - 38,
    //     };
    // }
    let result = shared_chars
        .iter()
        .map(|c| match c.is_ascii_lowercase() {
            true => *c as u16 - 96,
            false => *c as u16 - 38,
        })
        .sum::<u16>();
    println!("The total sum is {}", result);
    println!("Total found items should be 300/3 {}", shared_chars.len());
    // println!("The total sum is {}", sum);
    // println!("Average should be {}", ave/26);
    // println!("Should be {}", shared_chars.len());
}
