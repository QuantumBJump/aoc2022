use std::{time::{Instant, Duration}, fmt};
use std::rc::Rc;
use std::cell::RefCell;
use crate::util::{get_filename, read_lines};
extern crate separator;
use separator::Separatable;

const DEBUG: bool = false;

#[derive(Debug, Clone)]
pub enum Day7Error {
    CDError(String)
}

#[derive(Debug, Clone)]
enum FSObjectType {
    File(usize),
    Dir,
}

enum Command {
    Noop,
    Ls,
    Cd(String),
}

#[derive(Debug, Clone)]
struct Dir {
    name: String,
    contents: Vec<Rc<RefCell<Dir>>>,
    obj: FSObjectType,
    parent: Option<Rc<RefCell<Dir>>>,
    depth: usize,
    size: Option<usize>,
}

impl Dir {
    pub fn add_dir(&mut self, name: &str) -> Rc<RefCell<Dir>> {
        let new_dir = Rc::new(RefCell::new(Dir {
            name: name.to_string(),
            obj: FSObjectType::Dir,
            contents: vec![],
            parent: None,
            depth: self.depth+1,
            size: None,
        }));
        self.contents.push(Rc::clone(&new_dir));
        new_dir
    }

    fn add_file(&mut self, name: &str, size: usize) -> Rc<RefCell<Dir>> {
        let new_file = Rc::new(RefCell::new(Dir {
            name: name.to_string(),
            obj: FSObjectType::File(size),
            contents: vec![],
            parent: None,
            depth: self.depth+1,
            size: Some(size),
        }));
        self.contents.push(Rc::clone(&new_file));
        new_file
    }

    fn set_parent(&mut self, parent: Rc<RefCell<Dir>>) {
        self.parent = Some(parent);
    }

    fn cd(&self, command: &str) -> Result<Rc<RefCell<Dir>>, Day7Error> {
        match command {
            "/" => {
                if self.name == "/" {
                    return Err(Day7Error::CDError(String::from("Already in root dir")));
                }
                let mut curr_dir = self.cd("..").unwrap();
                loop {
                    let next_dir = curr_dir.borrow().cd("..");
                    match next_dir {
                        Ok(dir) => {curr_dir = dir;}
                        Err(_) => return Ok(curr_dir)
                    }
                }
            }
            ".." => {
                let new_dir = self.parent.as_ref();
                match new_dir {
                    None => return Err(Day7Error::CDError("Directory .. does not exist".to_string())),
                    Some(new_dir) => return Ok(Rc::clone(new_dir)),
                }
            }
            _ => {
                for item in &self.contents {
                    if item.borrow().name == command {
                        return Ok(Rc::clone(item));
                    }
                }
                return Err(Day7Error::CDError(format!("Directory {} does not exist", command)));
            }
        }
    }

    #[allow(dead_code)]
    fn get_size(&mut self) -> usize {
        match self.size{
            Some(size) => return size,
            None => {
                match self.obj {
                    FSObjectType::File(size) => {
                        self.size = Some(size);
                        return size;
                    }
                    FSObjectType::Dir => {
                        let mut res = 0;
                        for item in &self.contents {
                            res += item.borrow_mut().get_size();
                        }
                        self.size = Some(res);
                        return res;
                    }
                }
            }
        }
    }

    fn get_display(&self) -> String {
        let mut res = String::from("");
        if self.depth > 0 {
            for _ in 0..self.depth {
                res.push_str("  ");
            }
        }
        res.push_str("- ");
        match self.obj {
            FSObjectType::File(size) => {res.push_str(format!("{} ({})\n", self.name, size.separated_string()).as_str())}
            FSObjectType::Dir => {
                let size = self.size.unwrap();
                res.push_str(format!("{} (dir, {})\n", self.name, size.separated_string()).as_str());
                for item in &self.contents {
                    res.push_str(item.borrow_mut().get_display().as_str());
                }
            }
        }
        res
    }

    fn part_1_sizes(&mut self, depth: usize) -> usize {
        match self.obj {
            FSObjectType::File(_) => return 0,
            _ => {}
        }
        let mut total = 0;
        let size = self.get_size();
        if size < 100000 {
            total += size;
        }
        for item in &self.contents {
            total += item.borrow_mut().part_1_sizes(depth+1);
        }
        return total;
    }

    fn delete_size(&mut self, target: usize, smallest: usize, depth: usize) -> usize {
        match self.obj {
            FSObjectType::File(_) => { return smallest; }
            _ => {}
        }
        let our_size = self.get_size();
        let mut padding = String::from("");
        for _ in 0..=depth {
            padding.push_str(" ");
        }
        if DEBUG {
            print!("{}Checking dir {}, size {}... ",padding, self.name, our_size.separated_string());
        }
        let mut res = smallest;
        if our_size < target {
            if DEBUG {
                println!("unable to free {}", target.separated_string());
            }
            // If our own size is smaller than the target, neither we nor our children can help.
            return smallest;
        } else if our_size < smallest {
            // If our own size is enough to free up space but smaller than the current smallest, update it.
            if DEBUG {
                println!("Dir {} is size {}, smaller than {}", self.name, our_size.separated_string(), smallest.separated_string());
            }
            res = our_size;
        }
        // If we're big enough to help but still bigger than the smallest, maybe our children could help.
        let mut child_sizes:Vec<usize> = vec![res];
        for item in &self.contents {
            let child_size = item.borrow_mut().delete_size(target, res, depth+1);
            child_sizes.push(child_size);
            if child_size != res {
                if DEBUG {
                    println!("{}  Child {} is size {}",padding, item.borrow().name, child_size.separated_string());
                }
            }
        }
        let mut lowest = res;
        for item in child_sizes {
            if item < lowest {
                lowest = item;
            }
        }
        lowest
    }
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_display())
    }
}

fn child_dir(parent: Rc<RefCell<Dir>>, name: &str) {
    let child = parent.borrow_mut().add_dir(name);
    child.borrow_mut().set_parent(parent);
}

fn child_file(parent: Rc<RefCell<Dir>>, name: &str, size: usize) {
    let child = parent.borrow_mut().add_file(name, size);
    child.borrow_mut().set_parent(parent);
}

fn parse_cmd(line: String) -> Command {
    let mut words = line.split(" ");
    words.next();
    let cmd = words.next().unwrap();
    match cmd {
        "ls" => return Command::Ls,
        "cd" => {
            let dir = words.next().unwrap();
            return Command::Cd(dir.to_owned());
        }
        _ => return Command::Noop,
    }
}

pub fn part1(input: crate::Data) -> Result<(usize, Duration), Day7Error> {
    let now = Instant::now();
    let fs = Dir {
        name: String::from("/"),
        contents: Vec::new(),
        obj: FSObjectType::Dir,
        parent: None,
        depth: 0,
        size: None,
    };
    let root = Rc::new(RefCell::new(fs));
    let mut current = Rc::clone(&root);
    let mut cmd = Command::Noop;
    let lines = read_lines(get_filename("day7", input)).unwrap();
    for line in lines {
        if let Ok(line) = line {
            if line.starts_with("$") {
                cmd = parse_cmd(line);
                match cmd {
                    Command::Cd(ref dest) => {
                        let new_dir = current.borrow().cd(dest);
                        match new_dir {
                            Err(new_dir) => {
                                if current.borrow().name == "/" {
                                    continue;
                                } else {
                                    return Err(new_dir);
                                }
                            }
                            Ok(new_dir) => {
                                current = new_dir;
                            }
                        }
                    }
                    _ => {}
                }
            } else {
                match cmd {
                    Command::Noop => { continue; }
                    Command::Ls => {
                        let mut words = line.split(" ");
                        let file_type = words.next().unwrap();
                        let name = words.next().unwrap();
                        if file_type == "dir" {
                            child_dir(Rc::clone(&current), name);
                        } else {
                            let size = file_type.parse::<usize>().unwrap();
                            child_file(Rc::clone(&current), name, size);
                        }
                    }
                    Command::Cd(_) => {
                    }
                }
            }
        }
    }

    let new_dir = current.borrow().cd("/")?;
    current = new_dir;
    let size = current.borrow_mut().part_1_sizes(0);

    Ok((size, now.elapsed()))
}

pub fn part2(input: crate::Data) -> Result<(usize, Duration), Day7Error> {
    let now = Instant::now();
    let fs = Dir {
        name: String::from("/"),
        contents: Vec::new(),
        obj: FSObjectType::Dir,
        parent: None,
        depth: 0,
        size: None,
    };
    let root = Rc::new(RefCell::new(fs));
    let mut current = Rc::clone(&root);
    let mut cmd = Command::Noop;

    // Read in input
    let lines = read_lines(get_filename("day7", input)).unwrap();
    for line in lines {
        if let Ok(line) = line {
            if line.starts_with("$") {
                cmd = parse_cmd(line);
                match cmd {
                    Command::Cd(ref dest) => {
                        let new_dir = current.borrow().cd(dest);
                        match new_dir {
                            Err(new_dir) => {
                                if current.borrow().name == "/" {
                                    continue;
                                } else {
                                    return Err(new_dir);
                                }
                            }
                            Ok(new_dir) => {
                                current = new_dir;
                            }
                        }
                    }
                    _ => {}
                }
            } else {
                match cmd {
                    Command::Noop => { continue; }
                    Command::Ls => {
                        let mut words = line.split(" ");
                        let file_type = words.next().unwrap();
                        let name = words.next().unwrap();
                        if file_type == "dir" {
                            child_dir(Rc::clone(&current), name);
                        } else {
                            let size = file_type.parse::<usize>().unwrap();
                            child_file(Rc::clone(&current), name, size);
                        }
                    }
                    Command::Cd(_) => {
                    }
                }
            }
        }
    }

    let new_dir = current.borrow().cd("/")?;
    current = new_dir;

    let total_size = 70000000;
    let used_space = current.borrow_mut().get_size();
    let unused = total_size - used_space;
    let needed = 30_000_000;
    let to_free = needed - unused;

    if DEBUG {
        println!("{}\n", current.borrow());
        println!("Used {}/{}. To free: {}", used_space, total_size, to_free);
    }
    let size_to_del = current.borrow_mut().delete_size(to_free, used_space, 0);

    Ok((size_to_del, now.elapsed()))
}
