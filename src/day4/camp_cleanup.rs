

pub fn solve_part_2(input: &str) {
    let lines = input.lines();
    let mut answer = 0;
    for line in lines {
        let (first, second) = line.split_once(",").unwrap();

        let first_range = first.split_once("-").unwrap();
        let second_range = second.split_once("-").unwrap();

        let first_start = first_range.0.parse::<i32>().unwrap();
        let first_end = first_range.1.parse::<i32>().unwrap();
        let second_start = second_range.0.parse::<i32>().unwrap();
        let second_end = second_range.1.parse::<i32>().unwrap();

        if first_start >= second_start && second_end >= first_start {
            answer += 1;
        }else if second_start >= first_start && first_end >= second_start {
            answer += 1;
        }else{
        }
        
        

    }
    println!("The answer is {}", answer);



}

pub fn solve_part_1(input: &str) {
    let lines = input.lines();
    let mut answer = 0;
    for line in lines {
        let (first, second) = line.split_once(",").unwrap();

        let first_range = first.split_once("-").unwrap();
        let second_range = second.split_once("-").unwrap();

        let first_start = first_range.0.parse::<i32>().unwrap();
        let first_end = first_range.1.parse::<i32>().unwrap();
        let second_start = second_range.0.parse::<i32>().unwrap();
        let second_end = second_range.1.parse::<i32>().unwrap();

        if first_start <= second_start && first_end >= second_end {
           answer+=1;
        //    println!("{} : {} Added", line, answer);
        }else if second_start <= first_start && second_end >= first_end {
            answer+=1;
            // println!("{} : {} Added", line, answer);
        }else{
            // println!("{} : {}", line, answer);
        }
        
        

    }
    println!("The answer is {}", answer);



}