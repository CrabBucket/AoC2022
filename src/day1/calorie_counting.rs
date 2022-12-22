

pub fn run_part_1(input: &str) {
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

struct ElfTracker {
    most: i32,
    middle: i32,
    least: i32,
}
impl ElfTracker {
    pub fn new() -> Self {
        ElfTracker {
            most: -1,
            middle: -2,
            least: -3,
        }
    }
    pub fn try_insert(&mut self, calories: i32) -> bool{
        if calories > self.least {
            self.push(calories);
            return true;
        }else{
            return false;
        }
    }
    fn push(&mut self, num: i32){
        if num < self.middle {
            self.least = num;
        }else if num < self.most {
            self.least = self.middle;
            self.middle = num;
        }else {
            self.least = self.middle;
            self.middle= self.most;
            self.most = num;
        }
    }
    pub fn print_result(&self) {
        println!("Total of 3 Largest Elves: {}", self.least+self.middle+self.most)
    }
}



pub fn run_part_2(input: &str) {
    let mut elf_tracker = ElfTracker::new();
    let split = input.split("\n\n");
    for elf in split {
        let mut current_elf = 0;
        let elf_split = elf.split('\n');
        for calories in elf_split{
            current_elf+= match calories.parse::<i32>() {
                Ok(n) => n,
                Err(_) => 0,
            };
        }
        elf_tracker.try_insert(current_elf);
    }
    elf_tracker.print_result();

}

