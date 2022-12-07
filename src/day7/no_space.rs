use std::str::FromStr;
#[derive(Debug)]
pub struct DirectoryArena {
    pub dirs: Vec<Directory>,
}
impl DirectoryArena {
    pub fn new() -> DirectoryArena {
        DirectoryArena { dirs: Vec::new() }
    }
    pub fn add_dir(&mut self, name: String, size: u64, parent: Option<DirectoryId>) -> DirectoryId {
        let id = DirectoryId {
            name: self.dirs.len(),
        };
        self.dirs.push(Directory {
            name,
            size,
            parent,
            children: Vec::new(),
        });
        return id;
    }
    pub fn get(&self, id: DirectoryId) -> &Directory {
        &self.dirs[id.name]
    }
    pub fn get_mut(&mut self, id: DirectoryId) -> &mut Directory {
        &mut self.dirs[id.name]
    }
}

#[derive(Debug)]
pub struct Directory {
    pub name: String,
    pub size: u64,

    pub parent: Option<DirectoryId>,
    pub children: Vec<DirectoryId>,
}
impl Directory {
    pub fn add_file(&mut self, line: String) {
        let details: Vec<&str> = line.split_whitespace().collect();
        // println!("{}", line);
        let size = details[0].parse::<u64>().unwrap();
        self.size += size;
    }
    pub fn get_size(&self, arena: &DirectoryArena) -> u64 {
        return self.size
            + self
                .children
                .iter()
                .map(|child| arena.get(*child).get_size(arena))
                .sum::<u64>();
    }
}
#[derive(Debug, Copy, Clone)]
pub struct DirectoryId {
    pub name: usize,
}

enum Command {
    TraverseUp,
    TraverseDown(String),
}
impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "$ cd .." => Ok(Command::TraverseUp),
            _ => {
                let dir = s.split_whitespace().collect::<Vec<&str>>()[2];
                Ok(Command::TraverseDown(dir.to_string()))
            }
        }
    }
}

pub fn solve_part_1(input: &str) -> String {
    let mut lines = input.lines();
    let mut dir_arena = DirectoryArena::new();
    lines.next();
    lines.next();
    let mut line = lines.next();
    let mut current_dir = dir_arena.add_dir("/".to_string(), 0, None);

    while line.is_some() {
        let mut current_line = line.unwrap();

        if current_line.starts_with('$') {
            // println!("{}", current_line);
            let command = Command::from_str(current_line).unwrap();

            match command {
                Command::TraverseUp => {
                    // println!("current_dir: {:?}", current_dir);
                    current_dir = dir_arena.get(current_dir).parent.unwrap();
                    // println!("current_dir: {:?}", current_dir);
                }
                Command::TraverseDown(dir) => {
                    let dir_alias = dir_arena.get(current_dir);
                    // println!("cd: {}", dir);
                    // println!("current_dir: {:?}", &dir_alias);

                    let foundId = dir_alias
                        .children
                        .iter()
                        .find(|id| dir_arena.get(**id).name == dir)
                        .unwrap();

                    current_dir = *foundId;

                    // println!("current_dir: {:?}", current_dir);

                    lines.next();
                }
            }
        } else if current_line.starts_with('d') {
            let details: Vec<&str> = current_line.split_whitespace().collect();
            let name = details[1].to_string();
            let id = dir_arena.add_dir(name, 0, Some(current_dir));
            dir_arena.get_mut(current_dir).children.push(id);
        } else {
            dir_arena
                .get_mut(current_dir)
                .add_file(current_line.to_string());
        }
        line = lines.next();
    }
    print!("{:?}", dir_arena);
    dir_arena
        .dirs
        .iter()
        .filter(|dir| dir.get_size(&dir_arena) <= 100000)
        .map(|dir| dir.get_size(&dir_arena))
        .sum::<u64>()
        .to_string()
}

pub fn solve_part_2(input: &str) -> String {
    let mut lines = input.lines();
    let mut dir_arena = DirectoryArena::new();
    lines.next();
    lines.next();
    let mut line = lines.next();
    let mut current_dir = dir_arena.add_dir("/".to_string(), 0, None);

    while line.is_some() {
        let mut current_line = line.unwrap();

        if current_line.starts_with('$') {
            // println!("{}", current_line);
            let command = Command::from_str(current_line).unwrap();

            match command {
                Command::TraverseUp => {
                    // println!("current_dir: {:?}", current_dir);
                    current_dir = dir_arena.get(current_dir).parent.unwrap();
                    // println!("current_dir: {:?}", current_dir);
                }
                Command::TraverseDown(dir) => {
                    let dir_alias = dir_arena.get(current_dir);
                    // println!("cd: {}", dir);
                    // println!("current_dir: {:?}", &dir_alias);

                    let foundId = dir_alias
                        .children
                        .iter()
                        .find(|id| dir_arena.get(**id).name == dir)
                        .unwrap();

                    current_dir = *foundId;

                    // println!("current_dir: {:?}", current_dir);

                    lines.next();
                }
            }
        } else if current_line.starts_with('d') {
            let details: Vec<&str> = current_line.split_whitespace().collect();
            let name = details[1].to_string();
            let id = dir_arena.add_dir(name, 0, Some(current_dir));
            dir_arena.get_mut(current_dir).children.push(id);
        } else {
            dir_arena
                .get_mut(current_dir)
                .add_file(current_line.to_string());
        }
        line = lines.next();
    }
    // print!("{:?}", dir_arena);
    let total_used = dir_arena.get(DirectoryId { name: 0 }).get_size(&dir_arena);
    dir_arena
        .dirs
        .iter()
        .filter(|dir| dir.get_size(&dir_arena) > 30000000 - (70000000 - total_used))
        .reduce(|accum, item| {
            if accum.get_size(&dir_arena) <= item.get_size(&dir_arena) {
                accum
            } else {
                item
            }
        })
        .unwrap()
        .get_size(&dir_arena)
        .to_string()
}
