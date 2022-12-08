use std::io::prelude::*;
use std::io::BufReader;
use std::rc::Rc;
use std::cell::RefCell;

struct Dir {
    _name: String,
    size: usize,
    children: Vec<Rc<RefCell<Dir>>>,
    _parent: Option<Rc<RefCell<Dir>>>,
}

impl Dir {
    fn new(name: String, parent: Option<Rc<RefCell<Dir>>>, size: usize) -> Self {
        Self {
            _name: name,
            size: size,
            children: vec![],
            _parent: parent,
        }
    }

    fn increase_size(&mut self, size: usize) {
        self.size += size;
    }

    fn recursive_size(&self) -> usize {
        self.size + self.children.iter().map(|c| c.borrow().recursive_size()).sum::<usize>()
    }
}

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin);

    let lines: Vec<String> = reader
        .lines()
        .map(|l| {
            l.expect("line")
        })
        .collect();

    let mut root: Rc<RefCell<Dir>> = Rc::new(RefCell::new(Dir::new("".into(), None, 0)));
    let mut dir_stack: Vec<Rc<RefCell<Dir>>> = vec![];

    for line in lines {
        let split_line = line.split(" ").collect::<Vec<&str>>();
        match &split_line[..] {
            ["$", "cd", ".."] => {
                eprintln!("cd ..");
                dir_stack.pop();
            },
            ["$", "cd", "/"] => {
                eprintln!("cd /");
                let r = Rc::new(RefCell::new(Dir::new("/".into(), None, 0)));
                root = r.clone();
                dir_stack.push(r);
            }
            ["$", "cd", dir] => {
                eprintln!("cd {}", dir);
                let curr = dir_stack.last().unwrap();
                let child = Rc::new(RefCell::new(Dir::new((*dir).into(), Some(curr.clone()), 0)));
                {
                    curr.borrow_mut().children.push(child.clone());
                }
                dir_stack.push(child);
            },
            ["$", "ls"] => (),
            ["dir", _] => (),
            [file_size, _file] => {
                eprintln!("file size for {}: {}", _file, file_size);
                let size = file_size.parse::<usize>().expect("number");
                dir_stack.last().unwrap().borrow_mut().increase_size(size);
            },
            [] | [_] | [_, ..] => panic!("unexpected"),
        }
    }

    // Find dirs <= 100000 bytes and sum their sizes
    println!("size of root: {}", root.borrow().recursive_size());

    let mut size_sum = 0;
    dir_stack.clear();
    dir_stack.push(root.clone());
    while !dir_stack.is_empty() {
        let d = dir_stack.pop().expect("not empty");
        let size = d.borrow().recursive_size();
        if size <= 100000 {
            size_sum += size;
        }
        dir_stack.extend(d.borrow().children.clone());
    }

    println!("sum of dirs <= 10000 bytes: {}", size_sum);

    // Find smallest dir that's big enough to delete
    // FS has 70_000_000 bytes and needs at least 30_000_000 free
    let fs_size = 70_000_000;
    let required_free = 30_000_000;
    let current_free = fs_size - root.borrow().recursive_size();
    let required_to_delete = required_free - current_free;
    eprintln!("need to free {:?} bytes", required_to_delete);

    let mut min_deletable_size = required_free + 1;
    dir_stack.clear();
    dir_stack.push(root.clone());
    while !dir_stack.is_empty() {
        let d = dir_stack.pop().expect("not empty");
        let size = d.borrow().recursive_size();
        if size < min_deletable_size && size >= required_to_delete {
            min_deletable_size = size;
        }
        dir_stack.extend(d.borrow().children.clone());
    }

    println!("size of directory to delete: {} bytes", min_deletable_size);

    Ok(())
}
