use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use day19::Factory;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let lines = lines_from_file(filename).expect("Could not load");
    let mut factory = Factory::new();

    for line in lines {
        factory.add_line(&line);
    }
    println!("Answer: {}", factory.pt_1());
    println!("Answer: {}", factory.pt_2());
}
