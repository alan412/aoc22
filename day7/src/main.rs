use regex::RegexSet;
use std::{
    cell::RefCell,
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    rc::Rc,
};

use day7::DirEntry;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let lines = lines_from_file(filename).expect("Could not load");
    let set = RegexSet::new(&[
        r"\$ cd /",
        r"\$ cd \.\.",
        r"\$ cd (.+)",
        r"\$ ls",
        r"dir (.+)",
        r"(\d+) (.+)",
    ])
    .unwrap();
    let mut path: Vec<Rc<RefCell<DirEntry>>> = Vec::new();
    path.push(Rc::new(RefCell::new(DirEntry::new_dir("/".to_string()))));

    for line in lines {
        let matches = set.matches(&line);
        // println!("{:?} {}", matches, line);
        if matches.matched(0) {
            // go to root
            path.retain(|dir| dir.borrow().name == "/");
        } else if matches.matched(1) {
            _ = path.pop();
            // go up a directory
            println!("cd up a dir");
        } else if matches.matched(2) {
            let dir = &line[5..];
            println!("cd to {}", dir);
            let new_dir = path.last().unwrap().borrow().get_subdir(dir.to_string());
            println!("{:?}", new_dir);
            path.push(new_dir);
            // descend a directory
        } else if matches.matched(3) {
            // ls - ignore
            println!("ls");
        } else if matches.matched(4) {
            let dir = &line[4..];
            println!("Add dir {}", dir);

            let mut current_dir = path.last_mut().unwrap().borrow_mut();
            current_dir.add_new_dir(dir.to_string());
        } else if matches.matched(5) {
            let pieces: Vec<&str> = line.split_whitespace().collect();
            println!("{:?}", pieces);
            let mut current_dir = path.last_mut().unwrap().borrow_mut();
            current_dir.add_new_file(pieces[1].to_string(), pieces[0].parse::<u32>().unwrap());
        }
    }
    path[0].borrow().print_tree(0);
    let answer = path[0].borrow().get_less_than(100000);
    println!("Answer:{}", answer);
}
