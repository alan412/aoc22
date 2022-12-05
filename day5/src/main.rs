use day5::CrateStack;
use regex::Regex;
use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let lines = lines_from_file(filename).expect("Could not load");

    let mut in_begin = true;

    let num_stacks = 9;
    let mut crates: Vec<CrateStack> = Vec::with_capacity(num_stacks);
    for _ in 0..num_stacks {
        crates.push(CrateStack::new());
    }
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for line in lines {
        if in_begin {
            if line.len() == 0 {
                in_begin = false;
            } else {
                let mut pos = 1;
                let mut index = 0;
                let char_vec: Vec<char> = line.chars().collect();
                while pos < line.len() {
                    if char_vec[pos] != ' ' {
                        crates[index].add(char_vec[pos]);
                    }
                    pos += 4;
                    index += 1;
                }
            }
        } else {
            // directions
            let caps = re.captures(&line).unwrap();
            let qty: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
            let stack1: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
            let stack2: u32 = caps.get(3).unwrap().as_str().parse().unwrap();

            println!("Move {} from {} -> {}", qty, stack1, stack2);

            //crates[stack2 as usize].transfer(&mut crates[stack1 as usize], qty);
            for _ in 0..qty {
                let move_element = crates[(stack1 - 1) as usize].pop();
                crates[(stack2 - 1) as usize].push(move_element);
            }
        }
    }
    // println!("{:?}", crates);
    for c in crates {
        println!("{:?}", c.last());
    }
}
