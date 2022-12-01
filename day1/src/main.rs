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

    elves.sort_by(|a, b| b.get_total().cmp(&a.get_total()));

    println!(
        "{:?} {}",
        elves,
        elves[0].get_total() + elves[1].get_total() + elves[2].get_total()
    );
}
