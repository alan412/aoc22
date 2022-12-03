use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use day3::Rucksack;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let lines = lines_from_file(filename).expect("Could not load");

    let mut total = 0;

    for line in lines {
        let sack = Rucksack::new(line);
        let ch = sack.find_duplicated();
        let priority = Rucksack::get_priority(ch);
        total += priority;
        println!("Priority: {} {}", priority, total)
    }
}
