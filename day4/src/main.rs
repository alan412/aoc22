use regex::Regex;
use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use day4::CleaningAssignment;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let lines = lines_from_file(filename).expect("Could not load");

    let mut total = 0;

    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    for line in lines {
        let caps = re.captures(&line).unwrap();
        let ca_elf1 = CleaningAssignment::new(
            &caps.get(1).unwrap().as_str(),
            &caps.get(2).unwrap().as_str(),
        );
        let ca_elf2 = CleaningAssignment::new(
            &caps.get(3).unwrap().as_str(),
            &caps.get(4).unwrap().as_str(),
        );
        if ca_elf1.fully_contained(&ca_elf2) {
            total += 1;
        }

        println!("{:?} {:?} {}", ca_elf1, ca_elf2, total);
    }
}
