use pathfinding::prelude::astar;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, Ord, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct HeightMap {
    grid: Vec<Vec<i32>>,
    starting_pos: Point,
    target_pos: Point,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Point) -> u32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as u32
    }
}

impl HeightMap {
    pub fn new() -> Self {
        Self {
            grid: Vec::new(),
            starting_pos: Point { x: 0, y: 0 },
            target_pos: Point { x: 0, y: 0 },
        }
    }
    pub fn add_line(&mut self, line: String) {
        let row = self.grid.len() as i32;
        let mut new_line: Vec<i32> = Vec::new();
        let mut col = 0;

        for ch in line.chars() {
            match ch {
                'S' => {
                    self.starting_pos.x = col;
                    self.starting_pos.y = row;
                    new_line.push(0);
                }
                'E' => {
                    self.target_pos.x = col;
                    self.target_pos.y = row;
                    new_line.push(25);
                }
                'a'..='z' => new_line.push(ch as i32 - 'a' as i32),
                _ => panic!("Unknown character"),
            }
            col += 1;
        }
        self.grid.push(new_line);
    }
    fn successors(&self, pt: &Point) -> Vec<(Point, u32)> {
        let mut neighbors: Vec<(Point, u32)> = Vec::new();

        let max_height = self.grid[pt.y as usize][pt.x as usize] + 1;
        let max_y = self.grid.len() as i32 - 1;
        let max_x = self.grid[0].len() as i32 - 1;

        let mut new_points: Vec<Point> = Vec::new();

        if pt.x != 0 {
            new_points.push(Point::new(pt.x - 1, pt.y));
        }
        if pt.x != max_x {
            new_points.push(Point::new(pt.x + 1, pt.y));
        }
        if pt.y != 0 {
            new_points.push(Point::new(pt.x, pt.y - 1));
        }
        if pt.y != max_y {
            new_points.push(Point::new(pt.x, pt.y + 1));
        }
        for pt in new_points {
            if self.grid[pt.y as usize][pt.x as usize] <= max_height {
                neighbors.push((pt, 1));
            }
        }
        return neighbors;
    }
    pub fn find_path(&self) {
        let goal: Point = self.target_pos;
        let result = astar(
            &self.starting_pos,
            |p| self.successors(p),
            |p| p.distance(&goal) / 3,
            |p| *p == goal,
        );
        match result {
            None => println!("No path found :-("),
            Some((path, cost)) => println!("Path: {:?} cost: {}", path, cost),
        }
    }
}
