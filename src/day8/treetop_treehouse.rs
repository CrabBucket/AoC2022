struct Grid {
    pub trees: Vec<Vec<Tree>>,
    current: (u32, u32),
}
impl Grid {

    pub fn compute_scenic(&self, height: i32, x: usize, y: usize) -> u32 {
        let mut scenic = 0;
        let (mut yIndex, mut xIndex) = (y, x);

        let left_score = self.block_distance(height, x, y, Direction::Left);
        let right_score = self.block_distance(height, x, y, Direction::Right);
        let up_score = self.block_distance(height, x, y, Direction::Up);
        let down_score = self.block_distance(height, x, y, Direction::Down);

        scenic = left_score * right_score * up_score * down_score;

        return scenic;


    }

    pub fn block_distance(&self, height:i32, x: usize, y: usize, dir: Direction) -> u32 {
        match dir {
            Direction::Left => {
                if x == 0 {
                    return 0;
                }else if height <= self.trees[y][x-1].height{
                    return 1;
                
                }else {
                    return 1+self.block_distance(height, x-1, y, dir);
                }
            }
            Direction::Up => {
                if y == self.trees.len() - 1 {
                    return 0;
                }else if height <= self.trees[y+1][x].height{
                    return 1;
                
                }else {
                    return 1+self.block_distance(height, x, y+1, dir);
                }
            },
            Direction::Down => {
                if y == 0 {
                    return 0;
                }else if height <= self.trees[y-1][x].height{
                    return 1;
                
                }else {
                    return 1+ self.block_distance(height, x, y-1, dir);
                }
            },
            Direction::Right => {
                if x == self.trees[0].len() - 1 {
                    return 0;
                }else if height <= self.trees[y][x+1].height{
                    return 1;
                
                }else {
                    return 1 + self.block_distance(height, x+1, y, dir);
                }
            },
        }
    }


    pub fn new(input: String) -> Self {
        let mut tree_rows = Vec::new();
        for line in input.lines() {
            let mut tree_row = Vec::new();
            for character in line.chars() {
                tree_row.push(Tree::new(character.to_digit(10).unwrap() as i32));
            }
            tree_rows.push(tree_row);
        }
        Self {
            trees: tree_rows,
            current: (0, 0),
        }
    }

    pub fn cast_left(&mut self) {
        let (mut x, mut y) = (0, 0);

        while y < self.trees.len() {
            let mut max_height = -1;
            x=0;
            while x < self.trees.len() {
                let tree = &mut self.trees[y as usize][x as usize];
                if tree.height > max_height {
                    max_height = tree.height;
                } else {
                    tree.left_blocked = true;
                }
                x += 1;
            }
            y += 1;
        }
    }
    pub fn cast_right(&mut self) {
        let (mut x, mut y) = (self.trees[0].len() as i32 - 1, 0);

        while y < self.trees.len() {
            let mut max_height = -1;
            x = self.trees.len() as i32 - 1;
            while x >= 0 {
                let tree = &mut self.trees[y as usize][x as usize];
                if tree.height > max_height {
                    max_height = tree.height;
                } else {
                    tree.right_blocked = true;
                }
                x -= 1;
            }
            y += 1;
        }
    }
    pub fn cast_down(&mut self) {
        let (mut x, mut y) = (0, 0);


        while x < self.trees.len() {
            let mut max_height = -1;
            y = 0;
            while y < self.trees.len() {
                let tree = &mut self.trees[y as usize][x as usize];
                if tree.height > max_height {
                    max_height = tree.height;
                } else {
                    tree.down_blocked = true;
                }
                y += 1;
            }
            x += 1;
        }
    }
    pub fn cast_up(&mut self) {
        let (mut x, mut y) = (self.trees[0].len() as i32 - 1, self.trees.len() as i32 - 1);

        while x >= 0 {
            y =  self.trees.len() as i32-1;
            let mut max_height = -1;
            while y >= 0 {
                let tree = &mut self.trees[y as usize][x as usize];
                if tree.height > max_height {
                    max_height = tree.height;
                } else {
                    tree.up_blocked = true;
                }
                y -= 1;
            }
            x -= 1;
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Tree {
    height: i32,
    pub left_blocked: bool,
    pub right_blocked: bool,
    pub up_blocked: bool,
    pub down_blocked: bool,
}
impl Tree {
    pub fn new(height: i32) -> Self {
        Self {
            height,
            left_blocked: false,
            right_blocked: false,
            up_blocked: false,
            down_blocked: false,
        }
    }
}

pub fn solve_part_1(input: String) -> String {
    let mut grid = Grid::new(input);
    grid.cast_left();
    grid.cast_right();
    grid.cast_down();
    grid.cast_up();
    // println!("{:?}", grid.trees);

    let answer = grid.trees.iter().map(|row| {
        row.iter().filter(|tree| {
            !tree.left_blocked || !tree.right_blocked || !tree.up_blocked || !tree.down_blocked
        }).count()
    }).sum::<usize>().to_string();

    answer.to_string()
}

pub fn solve_part_2(input: String) -> String {
    let mut grid = Grid::new(input);
    let mut scenics = Vec::new();
    for y in 0..grid.trees.len() {
        for x in 0..grid.trees[0].len() {
            scenics.push(grid.compute_scenic(grid.trees[y][x].height,x, y));
        }
    }
    // println!("{:?}", scenics);
    scenics.sort();
    scenics.reverse();
    scenics[0].to_string()
}