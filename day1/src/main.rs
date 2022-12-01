use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use day1::Elf;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let lines = lines_from_file(filename).expect("Could not load");

    let mut elves: Vec<Elf> = Vec::new();

    let mut new_elf = Elf::new();
    for line in lines {
        if line.is_empty() {
            elves.push(new_elf);
            new_elf = Elf::new();
        } else {
            new_elf.add(line.parse::<i32>().unwrap());
        }
    }

    elves.push(new_elf);
    let mut totals: Vec<i32> = Vec::new();

    for elf in elves {
        totals.push(elf.get_total());
    }
    totals.sort();
    totals.reverse();

    println!("{:?} {}", totals, totals[0] + totals[1] + totals[2]);
}
