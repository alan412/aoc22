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
    fn in_bounds(&self, min: Self, max: Self) -> bool {
        self.x >= min.x - 1
            && self.x <= max.x + 1
            && self.y >= min.y - 1
            && self.y <= max.y + 1
            && self.z >= min.z - 1
            && self.z <= max.z + 1
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
    pub fn pt_1(&self) -> usize {
        self.cubes
            .iter()
            .flat_map(|cube| cube.neighbors())
            .filter(|cube| !self.cubes.contains(cube))
            .count()
    }
    pub fn pt_2(&self) -> usize {
        let exposed = self.exposed();
        self.cubes
            .iter()
            .flat_map(|cube| cube.neighbors())
            .filter(|cube| exposed.contains(cube))
            .count()
    }

    fn bounds(&self) -> (Cube, Cube) {
        let mut min = Cube::new(i32::MAX, i32::MAX, i32::MAX);
        let mut max = Cube::new(i32::MIN, i32::MIN, i32::MIN);
        for cube in self.cubes.iter() {
            min.x = min.x.min(cube.x);
            min.y = min.y.min(cube.y);
            min.z = min.z.min(cube.z);
            max.x = max.x.max(cube.x);
            max.y = max.y.max(cube.y);
            max.z = max.z.max(cube.z);
        }
        (min, max)
    }
    fn exposed(&self) -> HashSet<Cube> {
        let (min, max) = self.bounds();
        let mut exposed = HashSet::new();

        let start = Cube::new(0, 0, 0);
        let mut stack = Vec::new();
        let mut seen = HashSet::new();

        stack.push(start);
        seen.insert(start);

        while let Some(cube) = stack.pop() {
            for neighbor in cube.neighbors() {
                if self.cubes.contains(&neighbor) || !neighbor.in_bounds(min, max) {
                    continue;
                }
                if seen.insert(neighbor) {
                    stack.push(neighbor);
                    exposed.insert(neighbor);
                }
            }
        }
        exposed
    }
}
