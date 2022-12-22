use std::{collections::VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Recent {
    window: VecDeque<char>,
    index: u32,
    stream: String
}

impl Recent {
    pub fn new(input: &str, win_len: u32) -> Recent {
        let mut chars = input.chars();
        let mut window = VecDeque::new();
        for _ in 0..win_len {
            window.push_back(chars.next().unwrap());
        }
        Recent {
            window,
            index: win_len,
            stream: chars.collect()
        }
    }

    fn is_unique(&self) -> bool {
        let mut chars = self.window.iter();
        let mut unique = true;
        for _ in 0..self.window.len() {
            let c = chars.next().unwrap();
            if self.window.iter().filter(|&x| x == c).count() > 1 {
                unique = false;
                break;
            }
        }
        unique
    }
    fn next(&mut self) {
        self.window.pop_front();
        // println!("{}",self.stream);
        let mut temp = self.stream.chars();
        // println!("{}",self.stream);
        self.window.push_back(temp.next().unwrap());
        self.stream.remove(0);
        self.index += 1;
    }
    pub fn result(&self) -> u32 {
        self.index
    }
    pub fn window(&self) -> String {
        let mut chars = self.window.iter();
        let mut result = String::new();
        result.push('[');
        while let Some(c) = chars.next() {
            result.push(*c);
            result.push(' ');
        }
        result.push(']');
        result
    }

}

pub fn solve_part_1(input: &str) -> String {
    let mut recent = Recent::new(input,4);
    // println!("{:?}", recent);
    while !recent.is_unique() {
        // if loops > 10 {
        //     break;

        // }else{
        //     loops+=1;
        // }
        recent.next();
        println!("{}",recent.window())
    }
    recent.result().to_string()
    
}

pub fn solve_part_2(input: &str) -> String {
    let mut recent = Recent::new(input,14);
    // println!("{:?}", recent);
    while !recent.is_unique() {
        // if loops > 10 {
        //     break;

        // }else{
        //     loops+=1;
        // }
        recent.next();
        // println!("{}",recent.window())
    }
    recent.result().to_string()
    
}