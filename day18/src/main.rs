use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use day18::Droplet;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let lines = lines_from_file(filename).expect("Could not load");
    let mut droplet = Droplet::new();

    for line in lines {
        droplet.add_line(&line);
    }
    println!("Answer: {}", droplet.pt_1());
    println!("Answer: {}", droplet.pt_2());
}
