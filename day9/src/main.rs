use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use day9::Rope;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let lines = lines_from_file(filename).expect("Could not load");
    let mut rope = Rope::new();

    for line in lines {
        let v: Vec<String> = line.split_whitespace().map(str::to_string).collect();
        let distance = v[1].parse::<i32>().unwrap();
        println!("--{:?}", line);
        match v[0].as_str() {
            "L" => rope.left(distance),
            "R" => rope.right(distance),
            "D" => rope.down(distance),
            "U" => rope.up(distance),
            _ => panic!("Unknown direction"),
        }
    }
    println!("Answer: {}", rope.get_num_tail_visited());
}
