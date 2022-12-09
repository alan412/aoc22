use std::collections::HashMap;
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Rope {
    head: Point,
    tail: Point,
    tail_visited_points: HashMap<Point, u32>,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Rope {
    pub fn new() -> Self {
        let mut tail_visited_points = HashMap::new();
        tail_visited_points.insert(Point::new(0, 0), 1);
        Self {
            head: Point::new(0, 0),
            tail: Point::new(0, 0),
            tail_visited_points: tail_visited_points,
        }
    }
    fn does_tail_need_move(&self) -> bool {
        if (self.tail.x - self.head.x).abs() > 1 {
            true
        } else {
            if (self.tail.y - self.head.y).abs() > 1 {
                true
            } else {
                false
            }
        }
    }
    fn move_tail(&mut self) {
        let old_tail = self.tail;
        if self.head.x == self.tail.x {
            if self.head.y > self.tail.y {
                self.tail.y += 1
            } else {
                self.tail.y -= 1
            }
        } else if self.head.y == self.tail.y {
            if self.head.x > self.tail.x {
                self.tail.x += 1
            } else {
                self.tail.x -= 1
            }
        } else {
            if self.head.x < self.tail.x {
                self.tail.x -= 1;
                if self.head.y < self.tail.y {
                    self.tail.y -= 1;
                } else {
                    self.tail.y += 1;
                }
            } else {
                self.tail.x += 1;
                if self.head.y < self.tail.y {
                    self.tail.y -= 1;
                } else {
                    self.tail.y += 1;
                }
            }
        }
        println!(
            "Tail from {:?} to {:?} ({:?})",
            old_tail, self.tail, self.head
        )
    }

    fn move_head(&mut self, amount_x: i32, amount_y: i32) {
        self.head.x += amount_x;
        self.head.y += amount_y;
        if self.does_tail_need_move() {
            self.move_tail();
            self.tail_visited_points.entry(self.tail).or_insert(0);
            let num_times_visited = self.tail_visited_points.get(&self.tail).unwrap() + 1;
            _ = self
                .tail_visited_points
                .insert(self.tail, num_times_visited);
        }
    }

    pub fn up(&mut self, distance: i32) {
        for _ in 0..distance {
            self.move_head(0, 1);
        }
    }

    pub fn down(&mut self, distance: i32) {
        for _ in 0..distance {
            self.move_head(0, -1);
        }
    }

    pub fn left(&mut self, distance: i32) {
        for _ in 0..distance {
            self.move_head(-1, 0);
        }
    }

    pub fn right(&mut self, distance: i32) {
        for _ in 0..distance {
            self.move_head(1, 0);
        }
    }

    pub fn get_num_tail_visited(&self) -> usize {
        println!("{:?}", self.tail_visited_points);
        self.tail_visited_points.keys().len()
    }
}
// let val = m.entry(k).or_insert(d);