#[derive(Debug)]
pub struct Forest {
    grid: Vec<Vec<u32>>,
}

impl Forest {
    pub fn new() -> Self {
        Self { grid: Vec::new() }
    }
    pub fn add_line(&mut self, line: String) {
        let mut new_line: Vec<u32> = Vec::new();

        for ch in line.chars() {
            new_line.push(ch.to_digit(10).unwrap());
        }
        self.grid.push(new_line);
    }

    fn is_visible_from_left(&self, x: usize, y: usize) -> bool {
        let height = self.grid[y][x];

        for check_x in 0..x {
            if self.grid[y][check_x] >= height {
                return false;
            }
        }
        return true;
    }
    fn is_visible_from_top(&self, x: usize, y: usize) -> bool {
        let height = self.grid[y][x];

        for check_y in 0..y {
            if self.grid[check_y][x] >= height {
                return false;
            }
        }
        return true;
    }
    fn is_visible_from_right(&self, x: usize, y: usize) -> bool {
        let height = self.grid[y][x];

        for check_x in x + 1..self.grid.len() {
            if self.grid[y][check_x] >= height {
                return false;
            }
        }
        return true;
    }
    fn is_visible_from_bottom(&self, x: usize, y: usize) -> bool {
        let height = self.grid[y][x];

        for check_y in y + 1..self.grid.len() {
            if self.grid[check_y][x] >= height {
                return false;
            }
        }
        return true;
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        self.is_visible_from_left(x, y)
            || self.is_visible_from_top(x, y)
            || self.is_visible_from_right(x, y)
            || self.is_visible_from_bottom(x, y)
    }
    pub fn count_visible(&self) -> u32 {
        let size = self.grid.len();
        let mut total_visible = 0;
        for y in 0..size {
            for x in 0..size {
                total_visible += if self.is_visible(x, y) { 1 } else { 0 }
            }
        }
        total_visible
    }
    fn get_scenic_top(&self, x: usize, y: usize) -> u32 {
        let mut total_trees = 0;
        let height = self.grid[y][x];
        let mut new_y: i32 = y as i32 - 1;
        let new_x = x;

        while new_y >= 0 {
            if self.grid[new_y as usize][new_x] >= height {
                return total_trees + 1;
            } else {
                total_trees += 1;
                new_y -= 1;
            }
        }
        total_trees
    }
    fn get_scenic_bottom(&self, x: usize, y: usize) -> u32 {
        let mut total_trees = 0;
        let height = self.grid[y][x];
        let mut new_y = y + 1;
        let new_x = x;

        while new_y < self.grid.len() {
            if self.grid[new_y][new_x] >= height {
                return total_trees + 1;
            } else {
                total_trees += 1;
                new_y += 1;
            }
        }
        total_trees
    }
    fn get_scenic_left(&self, x: usize, y: usize) -> u32 {
        let mut total_trees = 0;
        let height = self.grid[y][x];
        let new_y = y;
        let mut new_x: i32 = x as i32 - 1;

        while new_x >= 0 {
            if self.grid[new_y][new_x as usize] >= height {
                return total_trees + 1;
            } else {
                total_trees += 1;
                new_x -= 1;
            }
        }
        total_trees
    }
    fn get_scenic_right(&self, x: usize, y: usize) -> u32 {
        let mut total_trees = 0;
        let height = self.grid[y][x];
        let new_y = y;
        let mut new_x = x + 1;

        while new_x < self.grid.len() {
            if self.grid[new_y][new_x] >= height {
                return total_trees + 1;
            } else {
                total_trees += 1;
                new_x += 1;
            }
        }
        total_trees
    }

    fn get_scenic(&self, x: usize, y: usize) -> u32 {
        let size = self.grid.len();

        if x == 0 || x == size - 1 || y == 0 || y == size - 1 {
            0
        } else {
            let total = self.get_scenic_top(x, y)
                * self.get_scenic_left(x, y)
                * self.get_scenic_right(x, y)
                * self.get_scenic_bottom(x, y);
            /*
                        println!(
                            "({} {}) {} {} {} {} {}",
                            x,
                            y,
                            self.get_scenic_top(x, y),
                            self.get_scenic_left(x, y),
                            self.get_scenic_right(x, y),
                            self.get_scenic_bottom(x, y),
                            total
                        );
            */
            total
        }
    }

    pub fn get_highest_scenic(&self) -> u32 {
        let size = self.grid.len();

        let mut largest = 0;
        for y in 0..size {
            for x in 0..size {
                let scenic = self.get_scenic(x, y);
                if scenic > largest {
                    largest = scenic;
                }
            }
        }
        largest
    }
}
