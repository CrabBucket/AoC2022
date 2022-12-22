enum MatchResult {
    Win,
    Lose,
    Draw,
}
impl MatchResult {
    pub fn new(input: &str) -> Self {
        if input == "A" || input == "X" {
            return MatchResult::Lose;
        }
        if input == "B" || input == "Y" {
            return MatchResult::Draw;
        }
        if input == "C" || input == "Z" {
            return MatchResult::Win;
        }
        panic!()
    }

    pub fn forced_move(&self, opp_move: &Move) -> Move{
        match self {
            MatchResult::Win => {
                match opp_move {
                    Move::Rock => Move::Paper,
                    Move::Paper => Move::Scissors,
                    Move::Scissors => Move::Rock,
                }
            },
            MatchResult::Lose => {
                match opp_move {
                    Move::Rock => Move::Scissors,
                    Move::Paper => Move::Rock,
                    Move::Scissors => Move::Paper,
                }
            },
            MatchResult::Draw => {
                match opp_move {
                    Move::Rock => Move::Rock,
                    Move::Paper => Move::Paper,
                    Move::Scissors => Move::Scissors,
                }
            },
        }
    }
}


enum Move {
    Rock,
    Paper,
    Scissors
}
impl Move {
    pub fn new(input: &str) -> Self {
        if input == "A" || input == "X" {
            return Move::Rock;
        }
        if input == "B" || input == "Y" {
            return Move::Paper;
        }
        if input == "C" || input == "Z" {
            return Move::Scissors;
        }
        panic!(); 
    }
    pub fn result(&self, other: &Move) -> i32 {
        let loss = 0;
        let draw = 3;
        let win = 6;
        let rock = 1;
        let paper = 2;
        let scissors = 3;
        match self {
            Move::Rock => {
                match other {
                    Move::Rock => draw + rock,
                    Move::Paper => loss + rock,
                    Move::Scissors => win + rock,
                }
            },
            Move::Paper => {
                match other {
                    Move::Rock => win + paper,
                    Move::Paper => draw + paper,
                    Move::Scissors => loss + paper,
                }
            },
            Move::Scissors => {
                match other {
                    Move::Rock => loss + scissors,
                    Move::Paper => win + scissors,
                    Move::Scissors => draw + scissors,
                }
            },
        }
    }
}
pub fn run_part_1(raw_input: &str) {
    let input = raw_input.trim_end();

    let mut total_points = 0;
    for round in input.split("\n"){
        let temp = round.split_whitespace();
        let moves: Vec<&str> = temp.collect();
        let opp_move = Move::new(moves[0]);
        let my_move = Move::new(moves[1]);

        total_points += my_move.result(&opp_move);
        
    }
    println!("With the Elf's strategy you would get: {}", total_points);
}
pub fn run_part_2(raw_input: &str) {
    let input = raw_input.trim_end();

    let mut total_points = 0;
    for round in input.split("\n"){
        let temp = round.split_whitespace();
        let moves: Vec<&str> = temp.collect();
        let opp_move = Move::new(moves[0]);
        let match_result = MatchResult::new(moves[1]);
        let my_move = match_result.forced_move(&opp_move);

        total_points += my_move.result(&opp_move);
        
    }
    println!("With the Elf's strategy you would get: {}", total_points);
}