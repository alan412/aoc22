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

    let mut elves: Vec<Rucksack> = Vec::new();
    for line in lines {
        let sack = Rucksack::new(line);
        elves.push(sack);
    }
    // For each set of 3, find the similarities
    let num_groups = elves.len() / 3;

    for group in 0..num_groups {
        let str1 = elves[group * 3].get_string();
        let str2 = elves[(group * 3) + 1].find_all_duplicates(&str1);
        let result = elves[(group * 3) + 2].find_all_duplicates(&str2);
        let ch = result.chars().next().unwrap();

        total += Rucksack::get_priority(ch);
        println!("Group {} ch{} result{} total:{}", group, ch, result, total);
    }
}
