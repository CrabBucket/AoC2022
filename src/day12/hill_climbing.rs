use std::{collections::{BTreeSet, VecDeque, BTreeMap}, panic::catch_unwind};



#[derive(Debug)]
struct HeightMap {
    map: Vec<Vec<u32>>,
    visited: BTreeMap<(usize, usize), u32>,
}
impl HeightMap {
    pub fn add(&mut self, y: usize, height: u32) {
        // println!("{} {}", y, self.map.len());
        if y as i32 > self.map.len() as i32 - 1 {
            self.map.push(Vec::new());
            self.add(y, height);
        }else{
            self.map[y].push(height);
        }
        
    }
    pub fn visit(&mut self, x: usize, y: usize, len: u32) {
        self.visited.insert((x, y),len);
    }
    pub fn valid_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        if x > 0 && self.map[y][x]+1  >= self.map[y][x - 1] && !self.visited.contains_key(&(x-1,y)){
            neighbors.push((x - 1, y));
        }
        if x < self.map[y].len() - 1 && self.map[y][x]+1 >= self.map[y][x + 1]&& !self.visited.contains_key(&(x+1,y)){
            neighbors.push((x + 1, y));
        }
        if y > 0 && self.map[y][x]+1 >= self.map[y - 1][x] && !self.visited.contains_key(&(x,y-1)){
            neighbors.push((x, y - 1));
        }
        if y < self.map.len() - 1 && self.map[y][x]+1 >= self.map[y + 1][x] && !self.visited.contains_key(&(x,y+1)) {
            neighbors.push((x, y + 1));
        }
        return neighbors;
    }
    pub fn valid_multi_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        // if x > 0 {
        //     println!("{} {} {} {}", x, y, self.map[y][x], self.map[y][x - 1]);
        // }
        if x > 0 && self.map[y][x]+1  >= self.map[y][x - 1] && (!self.visited.contains_key(&(x-1,y)) || self.map[y][x] < *self.visited.get(&(x-1,y)).unwrap()+1) {
            neighbors.push((x - 1, y));
        }
        if x < self.map[y].len() - 1 && self.map[y][x]+1 >= self.map[y][x + 1]&& (!self.visited.contains_key(&(x+1,y)) || self.map[y][x] < *self.visited.get(&(x+1,y)).unwrap()+1){
            neighbors.push((x + 1, y));
        }
        if y > 0 && self.map[y][x]+1 >= self.map[y - 1][x] && (!self.visited.contains_key(&(x,y-1)) || self.map[y][x] < *self.visited.get(&(x,y-1)).unwrap()+1){
            neighbors.push((x, y - 1));
        }
        if y < self.map.len() - 1 && self.map[y][x]+1 >= self.map[y + 1][x] && (!self.visited.contains_key(&(x,y+1)) || self.map[y][x] < *self.visited.get(&(x,y+1)).unwrap()+1) {
            neighbors.push((x, y + 1));
        }
        return neighbors;
    }
}
#[derive(Debug, Clone, Copy)]
struct Path {
    length: u32,
    current: (usize, usize),
}
impl Path {
    pub fn bfs(graph: &mut HeightMap, start: &Path, end: (usize,usize)) -> Path {
        let mut stack = VecDeque::new();
        stack.push_back(*start);
        graph.visit(start.current.0, start.current.1, start.length);
        while stack.len() > 0 {
            let current = stack.pop_front().unwrap();
            if current.current == end {
                return current;
            }
            for neighbor in graph.valid_neighbors(current.current.0, current.current.1) {
                let new_path = Path {
                    length: current.length + 1,
                    current: neighbor,
                };
                graph.visit(neighbor.0, neighbor.1, new_path.length);
                stack.push_back( new_path);
            }
        }
        return Path {
            length: 0,
            current: (0, 0),
        };
    }
    pub fn multi_entry_bfs(graph: &mut HeightMap, starts: Vec<Path>, end: (usize,usize)) -> Path {
        let mut stack = VecDeque::new();
        let mut paths = Vec::new();
        for path in starts {
            stack.push_back(path);
            graph.visit(path.current.0, path.current.1, path.length);
            while stack.len() > 0 {
                let current = stack.pop_front().unwrap();
                if current.current == end {
                    paths.push(current);
                    break;
                }
                for neighbor in graph.valid_multi_neighbors(current.current.0, current.current.1) {
                    let new_path = Path {
                        length: current.length + 1,
                        current: neighbor,
                    };
                    let temp = graph.visited.get(&(neighbor.0, neighbor.1)).unwrap_or(&0);
                    let me = graph.visited.get(&(current.current.0, current.current.1)).unwrap_or(&0);
                    println!("The length of the current path {:?} is: {}",current.current, me);
                    println!("The length of the neigbors path {:?} is: {}", neighbor,temp);
                    graph.visit(neighbor.0, neighbor.1, new_path.length);
                    stack.push_back( new_path);
                }
            }
            
        }
        if paths.len() > 0 {
            let mut shortest = paths[0];
            for path in paths {
                if path.length < shortest.length {
                    shortest = path;
                }
            }
            return shortest;
        }
        return Path {
            length: 0,
            current: (0, 0),
        };
    }
}

struct Parser {
    input: String,
}
impl Parser {
    fn new(input: String) -> Parser {
        Parser { input: input }
    }
    fn parse(&self) -> (HeightMap, (Path, (usize, usize))) {
        let mut x = 0;
        let mut y = 0;
        let mut terrain = HeightMap { map: Vec::new(), visited: BTreeMap::new() };
        let mut path = Path {
            length: 0,
            current: (0, 0),
        };
        let mut end = (999999999999999,9999999999999999);
        for line in self.input.lines() {
            for char in line.chars() {
                let mut height = char as u32;
                if char == 'S' {
                    path = Path {
                        length: 0,
                        current: (x, y),
                    };
                    height = 'a' as u32;
                }
                if char == 'E' {
                    end = (x, y);
                    height = 'z' as u32;
                }
                terrain.add(y, height as u32 - 'a' as u32);
                x += 1;
            }
            x = 0;
            y += 1;
        }
        return (terrain, (path, end));
    }
}

pub fn solve_part_1(input: String) -> String {
    let (mut terrain, start_end) = Parser::new(input).parse();
    println!("{:?}", terrain.valid_neighbors(5, 20));
    let result = Path::bfs(&mut terrain, &start_end.0, start_end.1);
    println!("{:?}", result);

    return "".to_string();
}

pub fn solve_part_2(input: String) -> String {
    let (mut terrain, start_end) = Parser::new(input).parse();
    let mut starts = Vec::new();
    let mut y = 0;
    for row in terrain.map.iter() {
       let mut x = 0;
       for height in row {
            if *height == 1 {
                starts.push(Path {
                    length: 1,
                    current: (x, y),
                });
            }
            x+=1;
       }
       y+=1;
    }
    println!("{:?}", starts);

    let result = Path::multi_entry_bfs(&mut terrain, starts, start_end.1);
    println!("{:?}", result);


    return "".to_string();
}