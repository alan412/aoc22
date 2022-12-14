use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, Ord, PartialOrd)]
struct Point {
    x: u32,
    y: u32,
}
#[derive(Debug)]
pub enum Material {
    Rock,
    Sand,
}

pub struct Cave {
    map: HashMap<Point, Material>,
    top_left: Point,
    bottom_right: Point,
}

fn parse_numbers(input: &str) -> IResult<&str, u32> {
    map_res(digit1, u32::from_str)(input)
}

fn range_inclusive(a: u32, b: u32) -> impl Iterator<Item = u32> {
    let x: Box<dyn Iterator<Item = u32>>;
    if b > a {
        x = Box::new(a..=b)
    } else {
        x = Box::new((b..=a).rev())
    }
    x
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
    fn parse(input: &str) -> IResult<&str, Self> {
        let parse_two_numbers = separated_pair(parse_numbers, char(','), parse_numbers);
        map(parse_two_numbers, |(x, y)| Point { x, y })(input)
    }
}

impl Cave {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            top_left: Point::new(u32::MAX, u32::MAX),
            bottom_right: Point::new(0, 0),
        }
    }
    pub fn add_line(&mut self, input: &str) {
        let result = separated_list1(tag(" -> "), Point::parse)(input);
        // go through result and add to hashmap
        let points = result.unwrap().1;
        let mut index: usize = 1;
        while index < points.len() {
            let start_x = points[index - 1].x;
            let start_y = points[index - 1].y;
            let end_x = points[index].x;
            let end_y = points[index].y;

            self.top_left.y = min(self.top_left.y, min(start_y, end_y));
            self.top_left.x = min(self.top_left.x, min(start_x, end_x));
            self.bottom_right.y = max(self.bottom_right.y, max(start_y, end_y));
            self.bottom_right.x = max(self.bottom_right.x, max(start_x, end_x));

            for x in range_inclusive(start_x, end_x) {
                self.map.insert(Point::new(x, start_y), Material::Rock);
            }
            for y in range_inclusive(start_y, end_y) {
                self.map.insert(Point::new(start_x, y), Material::Rock);
            }
            index += 1;
        }
    }
    fn get(&self, pt: &Point) -> Option<&Material> {
        if pt.y == self.bottom_right.y + 2 {
            Some(&Material::Rock)
        } else {
            self.map.get(pt)
        }
    }
    // True if the sand found a resting spot
    pub fn drop_sand(&mut self) -> bool {
        // For now always drops from 500, 0
        let mut sand_pos = Point::new(500, 0);

        loop {
            match self.get(&Point::new(sand_pos.x, sand_pos.y + 1)) {
                None => sand_pos.y += 1,
                Some(_) => match self.get(&Point::new(sand_pos.x - 1, sand_pos.y + 1)) {
                    None => {
                        sand_pos.x -= 1;
                        sand_pos.y += 1;
                    }
                    Some(_) => match self.get(&Point::new(sand_pos.x + 1, sand_pos.y + 1)) {
                        None => {
                            sand_pos.x += 1;
                            sand_pos.y += 1;
                        }
                        Some(_) => {
                            if sand_pos.y == 0 {
                                return false;
                            }
                            self.map.insert(sand_pos, Material::Sand);
                            return true;
                        }
                    },
                },
            }
        }
    }
    pub fn draw(&self) {
        for y in self.top_left.y..=self.bottom_right.y {
            let mut row = String::from("");
            for x in self.top_left.x..=self.bottom_right.x {
                row.push(match self.map.get(&Point::new(x, y)) {
                    Some(material) => match material {
                        Material::Rock => '#',
                        Material::Sand => 'o',
                    },
                    None => '.',
                });
            }
            println!("{} {}", y, row);
        }
    }
}
/*
    fn parse(line: &str) -> IResult<&str, Packet> {
        alt((
            delimited(
                complete::char('['),
                map(separated_list0(complete::char(','), parse), Packet::List),
                complete::char(']'),
            ),
            map(complete::u32, Packet::Int),
        ))(line)
    }
    let result = parse(input);
    result.unwrap().1
}
*/
