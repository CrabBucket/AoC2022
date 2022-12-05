use std::str::Lines;





struct SupplyStacks {
    stacks: Vec<Vec<char>>,
}
impl SupplyStacks {
    pub fn new(lines: &mut Lines, height: u32, width: u32) -> SupplyStacks {
        let mut stacks = Vec::<Vec<char>>::new();
        for _ in 0..width {
            stacks.push(Vec::<char>::new());
        }
        for line in 0..height {
            let line: Vec<char> = lines.next().unwrap().chars().collect();

            for bin in 0..width {
                if line[(bin*4+1) as usize] != ' '{
                    stacks[bin as usize].insert(0, line[(bin*4+1) as usize]);
                }
            }
            
        }
        SupplyStacks { stacks }
    }
    fn get_max_len(&self) -> usize {
        let mut max_len = 0;
        let mut current_len = 0;
        for stack in &self.stacks {
            current_len = stack.len();
            if current_len > max_len {
                max_len = current_len;
            }
        }
        max_len
    }
    pub fn move_stack(&mut self, from: usize, to: usize){
        let from_stack = &mut self.stacks[from];
        let from_top = from_stack.pop().unwrap();
        let to_stack = &mut self.stacks[to];
        
        to_stack.push(from_top);
    }
    pub fn move_n(&mut self, n: u32, from: usize, to: usize) {
        for _ in 0..n {
            self.move_stack(from, to);
        }
    }
    pub fn move_n_special(&mut self, n: u32, from: usize, to: usize) {
        
        let from_stack = &mut self.stacks[from];
        let mut temp_stack = Vec::<char>::new();
        let remove_index = from_stack.len() - n as usize;
        for iter in 0..n {
            let from_top = from_stack.remove(remove_index);
            
            temp_stack.push(from_top);
        }
        self.stacks[to].append(&mut temp_stack);

        
    }
    pub fn to_string(&self) -> String {
        let mut output = String::new();
        let mut string = String::new();
        for stack in 0..self.stacks.len() {
            string+= &(" ".to_owned() + &(stack+1).to_string() + "   ");
        }
        let mut str_stack = Vec::<String>::new();
        let max_len = self.get_max_len();
        for depth in 0..max_len {
            let mut row = String::new();
            for stack in 0..self.stacks.len() {
                if depth < self.stacks[stack].len() {
                    row+= &("[".to_owned() + &self.stacks[stack][depth].to_string() + "]  ");
                }else{
                    row+= &"     ".to_string();
                }
            }
            str_stack.push(row);
        }
        str_stack.reverse();
        for row in str_stack {
            output+= &(row.to_owned() + &"\n");
        }
        output+= &string;

        output.to_string()
    }

}


pub fn solve_part_1(input: &str) -> String {

    let mut lines = input.lines();
    
    let mut stacks = SupplyStacks::new(&mut lines, 8,9);
    println!("{}", stacks.to_string());
    lines.next().unwrap();
    lines.next().unwrap();
    let mut max = 30;
    let mut current = 0;
    for line in lines {
        let relevant_text = line.rsplitn(6, ' ').collect::<Vec<&str>>();
        let (n,from,to) = (relevant_text[4],relevant_text[2],relevant_text[0]);
        // println!("Move : {} From: {} To: {}", n, from, to);
        stacks.move_n(n.parse::<u32>().unwrap(), from.parse::<usize>().unwrap()-1, to.parse::<usize>().unwrap()-1);
        // println!("{}",stacks.to_string());

    }
    stacks.to_string()




}

pub fn solve_part_2(input: &str) -> String {

    let mut lines = input.lines();
    
    let mut stacks = SupplyStacks::new(&mut lines, 8,9);
    println!("{}", stacks.to_string());
    lines.next().unwrap();
    lines.next().unwrap();
    let mut max = 30;
    let mut current = 0;
    for line in lines {
        let relevant_text = line.rsplitn(6, ' ').collect::<Vec<&str>>();
        let (n,from,to) = (relevant_text[4],relevant_text[2],relevant_text[0]);
        // println!("Move : {} From: {} To: {}", n, from, to);
        stacks.move_n_special(n.parse::<u32>().unwrap(), from.parse::<usize>().unwrap()-1, to.parse::<usize>().unwrap()-1);
        // println!("{}",stacks.to_string());

    }
    stacks.to_string()




}