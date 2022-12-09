use std::collections::HashSet;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct MotionCommand {
    pub direction: Direction,
    pub distance: i32,
}
impl MotionCommand {
    pub fn new(input: String) -> MotionCommand {
        let temp = input.split_whitespace().collect::<Vec<&str>>();
        // println!("{:?}", temp);
        let direction = match temp[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction"),
        };
        let distance = temp[1].parse::<i32>().unwrap();
        MotionCommand {
            direction,
            distance,
        }
    }
}

struct PhysicsModel {
    pub head: Knot,
    pub body: Vec<Knot>,
    pub tail: Knot,
}
impl PhysicsModel {
    pub fn update(&mut self) {
        let mut cur_knot = &self.head;
        for knot in &mut self.body {
            if cur_knot.too_far(knot) {
                knot.follow(cur_knot);
            }
            cur_knot = knot;
        }
        if self.tail.too_far(cur_knot) {
            self.tail.follow(cur_knot);
            self.tail.visited.insert((self.tail.x, self.tail.y));
        }
    }
    pub fn new() -> PhysicsModel {
        let mut tail_visited = HashSet::new();
        tail_visited.insert((0, 0));
        PhysicsModel {
            head: Knot {
                x: 0,
                y: 0,
                visited: HashSet::new(),
            },
            body: Vec::new(),
            tail: Knot {
                x: 0,
                y: 0,
                visited: tail_visited,
            },
        }
    }
    pub fn move_n(&mut self, command: MotionCommand) {
        match command.direction {
            Direction::Up => {
                for _ in 0..command.distance {
                    self.head.y += 1;
                    self.update();
                }
            }
            Direction::Down => {
                for _ in 0..command.distance {
                    self.head.y -= 1;
                    self.update();
                }
            }
            Direction::Left => {
                for _ in 0..command.distance {
                    self.head.x -= 1;
                    self.update();
                }
            }
            Direction::Right => {
                for _ in 0..command.distance {
                    self.head.x += 1;
                    self.update();
                }
            }
        }
    }
}

struct Knot {
    pub x: i32,
    pub y: i32,
    pub visited: HashSet<(i32, i32)>,
}
impl Knot {
    pub fn too_far(&self, other: &Knot) -> bool {
        (self.x - other.x).abs() > 1 || (self.y - other.y).abs() > 1
    }
    pub fn follow(&mut self, other: &Knot) {
        if other.x > self.x {
            if other.y > self.y {
                self.y += 1;
                self.x += 1;
            } else if other.y < self.y {
                self.y -= 1;
                self.x += 1;
            } else {
                self.x += 1;
            }
        } else if other.x < self.x {
            if other.y > self.y {
                self.y += 1;
                self.x -= 1;
            } else if other.y < self.y {
                self.y -= 1;
                self.x -= 1;
            } else {
                self.x -= 1;
            }
        } else {
            if other.y > self.y {
                self.y += 1;
            } else if other.y < self.y {
                self.y -= 1;
            }
        }
        // println!("{} {}", self.x, self.y)
    }
}

pub fn solve_part_1(input: String) -> String {
    let mut model = PhysicsModel::new();
    let mut lines = input.lines();

    for line in lines {
        model.move_n(MotionCommand::new(line.to_string()));
    }
    println!("{:?}", model.tail.visited);
    model.tail.visited.len().to_string()
}

pub fn solve_part_2(input: String) -> String {
    let mut model = PhysicsModel::new();
    for _ in 1..9 {
        model.body.push(Knot {
            x: 0,
            y: 0,
            visited: HashSet::new(),
        });
    }
    let mut lines = input.lines();

    for line in lines {
        model.move_n(MotionCommand::new(line.to_string()));
    }
    // println!("{:?}", model.tail.visited);
    model.tail.visited.len().to_string()
}
