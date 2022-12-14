use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use day14::Cave;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let lines = lines_from_file(filename).expect("Could not load");
    let mut cave = Cave::new();

    for line in lines {
        cave.add_line(&line);
    }
    cave.draw();
    let mut total_sand = 0;
    while cave.drop_sand() {
        // cave.draw();
        total_sand += 1;
    }
    println!("Total Sand: {}", total_sand);
}
