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
    packets.push(Packet::new("[[2]]"));
    packets.push(Packet::new("[[6]]"));

    packets.sort();

    let divider1 = Packet::new("[[2]]");
    let divider2 = Packet::new("[[6]]");

    let mut index = 0;
    let mut answer = 0;
    while index < packets.len() {
        if packets[index] == divider1 {
            println!("Divider1 found at {}", index + 1);
            answer = index + 1;
        } else if packets[index] == divider2 {
            println!("Divider2 found at {}", index + 1);
            answer = answer * (index + 1);
        }
        index += 1;
    }
    println!("Answer: {}", answer);
}
