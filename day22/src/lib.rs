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

#[derive(Debug)]
enum Facing {
    Right,
    Left,
    Down,
    Up,
}

#[derive(Debug)]
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

    fn sub_y(&self, pt: &mut Point) {
        if pt.y == 0 {
            pt.y = self.rows - 1
        } else {
            pt.y = pt.y - 1
        }
    }
    fn sub_x(&self, pt: &mut Point) {
        if pt.x == 0 {
            pt.x = self.largest_x - 1
        } else {
            pt.x = pt.x - 1
        }
    }
    fn add_y(&self, pt: &mut Point) {
        pt.y += 1;
        if pt.y >= self.rows {
            pt.y = 0
        }
    }
    fn add_x(&self, pt: &mut Point) {
        pt.x += 1;
        if pt.x >= self.largest_x {
            pt.x = 0;
        }
    }

    fn move_self(&self, pt: Point, f: fn(&Self, &mut Point)) -> Point {
        let mut new_point = pt;
        f(self, &mut new_point);

        loop {
            match self.map.get(&new_point) {
                None => f(self, &mut new_point),
                Some(ch) => {
                    if *ch == '#' {
                        return pt;
                    } else {
                        return new_point;
                    }
                }
            }
        }
    }

    fn move_pos(&self, pos: &mut Position, step: &Step) {
        match step {
            Step::Left => pos.turn_left(),
            Step::Right => pos.turn_right(),
            Step::Move(amount) => {
                for _ in 0..*amount {
                    pos.pt = match pos.dir {
                        Facing::Left => self.move_self(pos.pt, Self::sub_x),
                        Facing::Right => self.move_self(pos.pt, Self::add_x),
                        Facing::Down => self.move_self(pos.pt, Self::add_y),
                        Facing::Up => self.move_self(pos.pt, Self::sub_y),
                    }
                }
            }
        }
    }

    pub fn pt_1(&self) -> u32 {
        let mut pos = Position {
            pt: self.find_upper_left(),
            dir: Facing::Right,
        };
        for step in self.steps.iter() {
            self.move_pos(&mut pos, step);
        }
        pos.score()
    }
    pub fn pt_2(&self) -> u32 {
        0
    }
}
