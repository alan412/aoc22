use std::{env, fs};

use day17::Cave;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");
    let mut cave = Cave::new(contents);

    //println!("{:?}", cave);
    //println!("Answer: {}", cave.part_1(2022));
    println!("Answer - Pt2: {}", cave.part_1(1_000_000_000_000));
}
