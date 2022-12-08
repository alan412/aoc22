use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use day8::Forest;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let lines = lines_from_file(filename).expect("Could not load");

    let mut forest = Forest::new();

    for line in lines {
        forest.add_line(line);
    }
    //println!("{:?} {}", forest, forest.count_visible());
    println!("{}", forest.get_highest_scenic());
}
