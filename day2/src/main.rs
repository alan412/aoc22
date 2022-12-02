use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

#[derive(Debug)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

fn calc_score(ours: RockPaperScissors, theirs: RockPaperScissors) -> i32 {
    println!("{:?} vs {:?}", ours, theirs);
    match ours {
        RockPaperScissors::Rock => match theirs {
            RockPaperScissors::Rock => 1 + 3,
            RockPaperScissors::Paper => 1 + 0,
            RockPaperScissors::Scissors => 1 + 6,
        },
        RockPaperScissors::Paper => match theirs {
            RockPaperScissors::Rock => 2 + 6,
            RockPaperScissors::Paper => 2 + 3,
            RockPaperScissors::Scissors => 2 + 0,
        },
        RockPaperScissors::Scissors => match theirs {
            RockPaperScissors::Rock => 3 + 0,
            RockPaperScissors::Paper => 3 + 6,
            RockPaperScissors::Scissors => 3 + 3,
        },
    }
}
fn convert_line(str: &String) -> (RockPaperScissors, RockPaperScissors) {
    let tokens: Vec<&str> = str.split_whitespace().collect();

    let first = match tokens[0] {
        "A" => RockPaperScissors::Rock,
        "B" => RockPaperScissors::Paper,
        "C" => RockPaperScissors::Scissors,
        _ => panic!("{}", tokens[0]),
    };
    let second = match tokens[1] {
        "X" => RockPaperScissors::Rock,
        "Y" => RockPaperScissors::Paper,
        "Z" => RockPaperScissors::Scissors,
        _ => panic!("{}", tokens[0]),
    };
    (first, second)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let lines = lines_from_file(filename).expect("Could not load");
    let mut total = 0;

    for line in lines {
        let (theirs, ours) = convert_line(&line);
        let score = calc_score(ours, theirs);
        total += score;
        println!("{} {} {}", line, score, total)
    }
}
