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
}
