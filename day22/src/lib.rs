use nom::{
    branch::alt,
    character::complete::{digit1, one_of},
    combinator::map,
    multi::many1,
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Point {
    x: u32,
    y: u32,
}
#[derive(Debug, Eq, PartialEq)]
enum Step {
    Move(u32),
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
enum Facing {
    Right,
    Left,
    Down,
    Up,
}

#[derive(Debug, Copy, Clone)]
struct Position {
    pt: Point,
    dir: Facing,
}

#[derive(Debug)]
pub struct Puzzle {
    rows: u32,
    largest_x: u32,
    map: HashMap<Point, char>,
    steps: Vec<Step>,
}

fn parse_direction(input: &str) -> IResult<&str, Step> {
    alt((
        map(digit1, |s: &str| {
            let num = s.parse::<u32>().unwrap();
            Step::Move(num)
        }),
        map(one_of("LR"), |c| match c {
            'L' => Step::Left,
            'R' => Step::Right,
            _ => unreachable!(),
        }),
    ))(input)
}
impl Position {
    fn turn_left(&mut self) {
        self.dir = match self.dir {
            Facing::Left => Facing::Down,
            Facing::Down => Facing::Right,
            Facing::Right => Facing::Up,
            Facing::Up => Facing::Left,
        }
    }
    fn turn_right(&mut self) {
        self.dir = match self.dir {
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
            Facing::Up => Facing::Right,
        }
    }
    fn score(&self) -> u32 {
        (1000 * (self.pt.y + 1))
            + (4 * (self.pt.x + 1))
            + match self.dir {
                Facing::Right => 0,
                Facing::Down => 1,
                Facing::Left => 2,
                Facing::Up => 3,
            }
    }
}
impl Puzzle {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            rows: 0,
            steps: Vec::new(),
            largest_x: 0,
        }
    }
    pub fn add_map_line(&mut self, line: &str) {
        for (i, c) in line.chars().enumerate() {
            match c {
                '.' | '#' => {
                    self.map.insert(
                        Point {
                            x: i as u32,
                            y: self.rows as u32,
                        },
                        c,
                    );
                    self.largest_x = self.largest_x.max(i as u32);
                }
                ' ' => {}
                _ => unreachable!(),
            };
        }
        self.rows += 1;
    }
    pub fn add_directions(&mut self, line: &str) {
        let result = many1(parse_direction)(line).unwrap();
        for step in result.1 {
            self.steps.push(step);
        }
    }
    pub fn add_line(&mut self, line: &str) {
        if line == "" {
            return;
        }
        match &line[..1] {
            "." | "#" | " " => self.add_map_line(line),
            _ => self.add_directions(line),
        }
    }
    fn find_upper_left(&self) -> Point {
        let mut x = 0;
        loop {
            let pt = Point { x, y: 0 };
            match self.map.get(&pt) {
                Some(ch) => {
                    if *ch == '.' {
                        return pt;
                    }
                }
                None => {}
            }
            x += 1;
        }
    }
    fn move_one_2d(&self, pos: &Position) -> Position {
        match pos.dir {
            Facing::Right => Position {
                pt: Point {
                    x: (pos.pt.x + 1) % self.largest_x,
                    y: pos.pt.y,
                },
                dir: pos.dir,
            },
            Facing::Down => Position {
                pt: Point {
                    x: pos.pt.x,
                    y: (pos.pt.y + 1) % self.rows,
                },
                dir: pos.dir,
            },
            Facing::Left => Position {
                pt: Point {
                    x: pos.pt.x.checked_sub(1).unwrap_or(self.largest_x - 1),
                    y: pos.pt.y,
                },
                dir: pos.dir,
            },
            Facing::Up => Position {
                pt: Point {
                    x: pos.pt.x,
                    y: pos.pt.y.checked_sub(1).unwrap_or(self.rows - 1),
                },
                dir: pos.dir,
            },
        }
    }

    // only works for cube in this order
    // _AB
    // _C_
    // DE_
    // F__
    // A on top

    fn move_cube(&self, pos: &Position) -> Position {
        let x = pos.pt.x;
        let y = pos.pt.y;

        //println!("pos: {:?}", pos);
        match pos.dir {
            Facing::Right => match (x, y) {
                (99, 0..=49) => Position {
                    pt: Point { x: 100, y },
                    dir: Facing::Right,
                },
                (149, 0..=49) => Position {
                    pt: Point { x: 99, y: 149 - y },
                    dir: Facing::Left,
                },
                (99, 50..=99) => Position {
                    pt: Point { x: 50 + y, y: 49 },
                    dir: Facing::Up,
                },
                (49, 100..=149) => Position {
                    pt: Point { x: 50, y },
                    dir: Facing::Right,
                },
                (99, 100..=149) => Position {
                    pt: Point { x: 149, y: 149 - y },
                    dir: Facing::Left,
                },
                (49, 150..=199) => Position {
                    pt: Point { x: y - 100, y: 149 },
                    dir: Facing::Up,
                },
                _ => Position {
                    pt: Point { x: x + 1, y },
                    dir: Facing::Right,
                },
            },
            Facing::Left => match (x, y) {
                (50, 0..=49) => Position {
                    pt: Point { x: 0, y: 149 - y },
                    dir: Facing::Right,
                },
                (100, 0..=49) => Position {
                    pt: Point { x: 99, y },
                    dir: Facing::Left,
                },
                (50, 50..=99) => Position {
                    pt: Point { x: y - 50, y: 100 },
                    dir: Facing::Down,
                },
                (0, 100..=149) => Position {
                    pt: Point { x: 50, y: 149 - y },
                    dir: Facing::Right,
                },
                (50, 100..=149) => Position {
                    pt: Point { x: 49, y },
                    dir: Facing::Left,
                },
                (0, 150..=199) => Position {
                    pt: Point { x: y - 100, y: 0 },
                    dir: Facing::Down,
                },
                _ => Position {
                    pt: Point { x: x - 1, y },
                    dir: Facing::Left,
                },
            },
            Facing::Up => match (x, y) {
                (50..=99, 0) => Position {
                    pt: Point { x: 0, y: 100 + x },
                    dir: Facing::Right,
                },
                (100..=149, 0) => Position {
                    pt: Point { x: x - 100, y: 199 },
                    dir: Facing::Up,
                },
                (50..=99, 50) => Position {
                    pt: Point { x, y: 49 },
                    dir: Facing::Up,
                },
                (0..=49, 100) => Position {
                    pt: Point { x: 50, y: x + 50 },
                    dir: Facing::Right,
                },
                (50..=99, 100) => Position {
                    pt: Point { x, y: 99 },
                    dir: Facing::Up,
                },
                (0..=49, 150) => Position {
                    pt: Point { x, y: 149 },
                    dir: Facing::Up,
                },
                _ => Position {
                    pt: Point { x, y: y - 1 },
                    dir: Facing::Up,
                },
            },
            Facing::Down => match (x, y) {
                (50..=99, 49) => Position {
                    pt: Point { x, y: 50 },
                    dir: Facing::Down,
                },
                (100..=149, 49) => Position {
                    pt: Point { x: 99, y: x - 50 },
                    dir: Facing::Left,
                },
                (50..=99, 99) => Position {
                    pt: Point { x, y: 100 },
                    dir: Facing::Down,
                },
                (0..=49, 149) => Position {
                    pt: Point { x, y: 150 },
                    dir: Facing::Down,
                },
                (50..=99, 149) => Position {
                    pt: Point { x: 49, y: 100 + x },
                    dir: Facing::Left,
                },
                (0..=49, 199) => Position {
                    pt: Point { x: x + 100, y: 0 },
                    dir: Facing::Down,
                },
                _ => Position {
                    pt: Point { x, y: y + 1 },
                    dir: Facing::Down,
                },
            },
        }
    }

    fn move_self(&self, pos: &Position, move_one: fn(&Self, &Position) -> Position) -> Position {
        let mut new_position = *pos;
        new_position = move_one(self, &new_position);
        println!("{:?} -> {:?}", pos, new_position);

        loop {
            match self.map.get(&new_position.pt) {
                None => {
                    unreachable!("Huh?");
                    new_position = move_one(self, &new_position)
                }
                Some(ch) => {
                    if *ch == '#' {
                        return *pos;
                    } else {
                        println!("Returning {:?} -> {:?}", pos, new_position);
                        return new_position;
                    }
                }
            }
        }
    }

    fn solve(&self, move_one: fn(&Self, &Position) -> Position) -> u32 {
        let mut pos = Position {
            pt: self.find_upper_left(),
            dir: Facing::Right,
        };

        for step in self.steps.iter() {
            println!("Step: {:?} Pos: {:?}", step, pos);
            match step {
                Step::Left => pos.turn_left(),
                Step::Right => pos.turn_right(),
                Step::Move(amount) => {
                    for i in 0..*amount {
                        println!("-{} {:?}", i, pos);
                        pos = self.move_self(&pos, move_one);
                        println!("--{} {:?}", i, pos);
                    }
                }
            }
        }
        pos.score()
    }

    pub fn pt_1(&self) -> u32 {
        0
        // self.solve(Self::move_one_2d)
    }

    pub fn pt_2(&self) -> u32 {
        self.solve(Self::move_cube)
    }
}
