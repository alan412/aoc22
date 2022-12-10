use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use day10::Cpu;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let lines = lines_from_file(filename).expect("Could not load");
    let mut cpu = Cpu::new();

    for line in lines {
        cpu.add_line(line);
    }
    cpu.execute();
    let mut total = 0;
    for cycle in [20, 60, 100, 140, 180, 220] {
        let x = cpu.get_x_at(cycle);
        println!("x at {} = {} ({})", cycle, x, cycle * x);
        total += x * cycle;
    }
    println!("Total: {}", total);
}
