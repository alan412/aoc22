use regex::Regex;
use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use day15::SensorNetwork;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let lines = lines_from_file(filename).expect("Could not load");
    let mut sensor_network = SensorNetwork::new();

    let re = Regex::new(
        r"Sensor at x=(\-?\d+), y=(\-?\d+): closest beacon is at x=(\-?\d+), y=(\-?\d+)",
    )
    .unwrap();

    for line in lines {
        let caps = re.captures(&line).unwrap();
        sensor_network.add_line(
            caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
            caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
        );
    }
    //println!("{:?}", sensor_network);
    //println!("{}", sensor_network.cannot_be(2000000));
    //println!("{}", sensor_network.get_frequency((40)));
    println!("{}", sensor_network.get_frequency(4_000_000));
}
