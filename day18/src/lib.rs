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
    cubes: HashSet<Cube>,
}

enum Dimension {
    X,
    Y,
    Z,
}

impl Cube {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
    fn neighbors(&self) -> Vec<Cube> {
        let mut neighbors = Vec::new();

        for dimension in [Dimension::X, Dimension::Y, Dimension::Z] {
            for offset in [-1, 1] {
                let mut neighbor = self.clone();
                match dimension {
                    Dimension::X => neighbor.x += offset,
                    Dimension::Y => neighbor.y += offset,
                    Dimension::Z => neighbor.z += offset,
                }
                neighbors.push(neighbor);
            }
        }
        neighbors
    }
}

impl Droplet {
    pub fn new() -> Self {
        Self {
            cubes: HashSet::new(),
        }
    }
    pub fn add_line(&mut self, line: &String) {
        let result: IResult<&str, Vec<i32>> =
            separated_list1(tag(","), map_res(digit1, i32::from_str))(line);
        let numbers = result.unwrap().1;
        self.cubes
            .insert(Cube::new(numbers[0], numbers[1], numbers[2]));
    }
    pub fn pt_1(&mut self) -> usize {
        self.cubes
            .iter()
            .flat_map(|cube| cube.neighbors())
            .filter(|cube| !self.cubes.contains(cube))
            .count()
    }
}
