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
    let mut line = String::new();

    for cycle in 0..240 {
        let remainder = cycle % 40;
        if remainder == 0 {
            println!("{}", line);
            line = String::new();
        }
        let x = cpu.get_x_at(cycle + 1);
        let ch = if (x == remainder - 1) || (x == remainder) || (x == remainder + 1) {
            '#'
        } else {
            '.'
        };
        line.push(ch);
    }
    println!("{}", line);
}
