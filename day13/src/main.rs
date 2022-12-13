use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use day13::Packet;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let lines = lines_from_file(filename).expect("Could not load");
    let mut packets: Vec<Packet> = Vec::new();

    for line in lines {
        if line.trim() != "" {
            packets.push(Packet::new(&line));
        }
    }
    let mut start_pair = 0;
    let mut pair_num = 1;
    let mut total = 0;
    while start_pair < packets.len() {
        if packets[start_pair] < packets[start_pair + 1] {
            println!("Correct {}", pair_num);
            total += pair_num;
        } else {
            println!("Wrong {}", pair_num);
        }
        pair_num += 1;
        start_pair += 2;
    }
    println!("Total: {}", total);
}
