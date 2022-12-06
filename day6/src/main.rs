use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn find_pattern(datastream: &Vec<char>) -> usize {
    let mut index = 0;

    while index < datastream.len() - 13 {
        let mut a: HashSet<char> = HashSet::new();
        let mut found = true;

        for i in index..index + 14 {
            if a.insert(datastream[i]) == false {
                found = false;
                break;
            }
        }
        if found {
            return index + 14;
        }
        index += 1;
    }
    panic!("Didn't find unique set!!")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let lines = lines_from_file(filename).expect("Could not load");
    for line in lines {
        let datastream: Vec<char> = line.chars().collect();
        println!("{:?} {}", datastream, find_pattern(&datastream));
    }
}
