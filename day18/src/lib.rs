use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::separated_list1,
    IResult,
};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
pub struct Droplet {
    cubes: Vec<Cube>,
}

impl Cube {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
    fn adjacent(&self, other: &Self) -> bool {
        self.adjacent_x(other) || self.adjacent_y(other) || self.adjacent_z(other)
    }
    fn adjacent_x(&self, other: &Self) -> bool {
        if self.y == other.y && self.z == other.z {
            [-1, 1].into_iter().any(|d| self.x + d == other.x)
        } else {
            false
        }
    }
    fn adjacent_y(&self, other: &Self) -> bool {
        if self.x == other.x && self.z == other.z {
            [-1, 1].into_iter().any(|d| self.y + d == other.y)
        } else {
            false
        }
    }
    fn adjacent_z(&self, other: &Self) -> bool {
        if self.x == other.x && self.y == other.y {
            [-1, 1].into_iter().any(|d| self.z + d == other.z)
        } else {
            false
        }
    }
}

impl Droplet {
    pub fn new() -> Self {
        Self { cubes: Vec::new() }
    }
    pub fn add_line(&mut self, line: &String) {
        let result: IResult<&str, Vec<i32>> =
            separated_list1(tag(","), map_res(digit1, i32::from_str))(line);
        let numbers = result.unwrap().1;
        self.cubes
            .push(Cube::new(numbers[0], numbers[1], numbers[2]));
    }
    pub fn pt_1(&mut self) -> u32 {
        let mut total = 0;
        for cube in self.cubes.iter() {
            let mut sides = 6;
            for other_cube in self.cubes.iter() {
                if cube == other_cube {
                    continue;
                }
                if cube.adjacent(other_cube) {
                    sides -= 1;
                }
            }
            total += sides
        }
        total
    }
}
