use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone, Ord, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone)]
enum Blizzard {
    Up,
    Down,
    Left,
    Right,
}

type BlizzardMap = BTreeMap<Point, Vec<Blizzard>>;

#[derive(Debug)]
pub struct Puzzle {
    blizzards: BlizzardMap,
    cache_blizzards: HashMap<u32, BlizzardMap>,
    cache_steps_left: HashMap<(u32, Point), u32>,
    line_num: usize,
    start_pt: Point,
    end_pt: Point,
    width: usize,
    best_time: u32,
}

impl Blizzard {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Blizzard::Up,
            '<' => Blizzard::Left,
            '>' => Blizzard::Right,
            'v' => Blizzard::Down,
            _ => unreachable!(),
        }
    }
}

impl Puzzle {
    pub fn new() -> Self {
        Self {
            blizzards: BTreeMap::new(),
            line_num: 0,
            start_pt: Point { x: 0, y: 0 },
            end_pt: Point { x: 0, y: 0 },
            width: 0,
            cache_blizzards: HashMap::new(),
            cache_steps_left: HashMap::new(),
            best_time: u32::MAX,
        }
    }

    pub fn add_line(&mut self, line: &str) {
        if self.line_num == 0 {
            self.width = line.len() - 2;
        }
        for (i, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    if self.line_num == 0 {
                        self.start_pt.x = i;
                    } else {
                        self.end_pt.x = i;
                        self.end_pt.y = self.line_num;
                    }
                }
                '#' => {}
                '^' | '<' | '>' | 'v' => self
                    .blizzards
                    .entry(Point {
                        x: i,
                        y: self.line_num,
                    })
                    .or_default()
                    .push(Blizzard::from_char(c)),
                _ => unreachable!(),
            };
        }
        self.line_num += 1;
    }
    fn move_blizzard(&self, blizzard: &Blizzard, pt: &Point) -> Point {
        let mut new_pt = *pt;
        match blizzard {
            Blizzard::Up => {
                new_pt.y -= 1;
                if new_pt.y == 0 {
                    new_pt.y = self.line_num - 1;
                }
            }
            Blizzard::Down => {
                new_pt.y += 1;
                if new_pt.y == self.line_num - 1 {
                    new_pt.y = 1;
                }
            }
            Blizzard::Left => {
                new_pt.x -= 1;
                if new_pt.x == 0 {
                    new_pt.x = self.width;
                }
            }
            Blizzard::Right => {
                new_pt.x += 1;
                if new_pt.x > self.width {
                    new_pt.x = 1;
                }
            }
        };
        new_pt
    }
    fn move_blizzards(&self, blizzards: &BlizzardMap) -> BlizzardMap {
        let mut new_blizzards: BlizzardMap = BlizzardMap::new();

        for (pt, list_blizzard) in blizzards.iter() {
            for blizzard in list_blizzard {
                let new_pt = self.move_blizzard(blizzard, &pt);
                new_blizzards.entry(new_pt).or_default().push(*blizzard);
            }
        }
        new_blizzards
    }

    fn can_move(&self, blizzards: &BlizzardMap, new_pt: Point) -> bool {
        if new_pt == self.end_pt {
            return true;
        }
        if new_pt.x == 0 || new_pt.x > self.width {
            return false;
        }
        if new_pt.y == 0 || new_pt.y >= (self.line_num - 1) {
            return false;
        }
        match blizzards.get(&new_pt) {
            Some(_) => false,
            None => true,
        }
    }

    fn solve(&mut self, min: u32, curr_pt: Point) -> u32 {
        let mut best = u32::MAX;
        if let Some(val) = self.cache_steps_left.get(&(min, curr_pt)) {
            return *val;
        }
        if min > self.best_time {
            return u32::MAX; // go ahead and give up, this can't be better
        }
        if curr_pt == self.end_pt {
            best = 0;
            self.best_time = min;
            println!("Found solution: {} {:?}", min, curr_pt);
        } else {
            //println!("Solving: {} {:?}", min, curr_pt);
            let next_min = min + 1;
            if let None = self.cache_blizzards.get(&next_min) {
                let curr_blizzard = self.cache_blizzards.get(&min).unwrap();
                let new_blizzards = self.move_blizzards(curr_blizzard);
                self.cache_blizzards.insert(next_min, new_blizzards);
            }
            let blizzards = match self.cache_blizzards.get(&next_min) {
                Some(x) => x,
                None => unreachable!(),
            };
            let mut attempts: Vec<Point> = Vec::new();
            let mut new_pt = curr_pt.clone();
            new_pt.y += 1; // Down
            if self.can_move(blizzards, new_pt) {
                attempts.push(new_pt);
            }
            new_pt = curr_pt;
            new_pt.x += 1; // Right
            if self.can_move(blizzards, new_pt) {
                attempts.push(new_pt);
            }
            new_pt = curr_pt;
            new_pt.x -= 1; // Left
            if self.can_move(blizzards, new_pt) {
                attempts.push(new_pt);
            }
            if new_pt.y != 0 {
                new_pt = curr_pt;
                new_pt.y -= 1; // Up
                if self.can_move(blizzards, new_pt) {
                    attempts.push(new_pt);
                }
            }
            if curr_pt == self.start_pt || self.can_move(blizzards, curr_pt) {
                attempts.push(curr_pt);
            }

            let mut best_pt = Point { x: 0, y: 0 };
            for pt in attempts {
                let result = self.solve(min + 1, pt);
                if result != u32::MAX {
                    best = best.min(result + 1);
                    best_pt = pt;
                }
            }
            if best != u32::MAX {
                println!("Min:{} Point:{:?}", min, best_pt);
            }
        }
        self.cache_steps_left.insert((min, curr_pt), best);
        best
    }
    pub fn pt_1(&mut self) -> u32 {
        self.cache_blizzards.insert(0, self.blizzards.clone());
        self.solve(0, self.start_pt)
    }
    pub fn pt_2(&mut self) -> u32 {
        let first_path = self.solve(0, self.start_pt);
        println!("First path: {}", first_path);
        let tmp_pt = self.start_pt;
        self.start_pt = self.end_pt;
        self.end_pt = tmp_pt;
        self.cache_steps_left.clear();
        self.best_time = u32::MAX;
        self.blizzards = self.cache_blizzards.get(&first_path).unwrap().clone();
        self.cache_blizzards.clear();
        let path_back = self.pt_1();
        println!("Path back: {}", path_back);
        let path_back_again = 0;
        /*
                self.end_pt = self.start_pt;
                self.start_pt = tmp_pt;
                self.cache_steps_left.clear();
                self.best_time = u32::MAX;
                self.blizzards = self.cache_blizzards.get(&path_back).unwrap().clone();
                self.cache_blizzards.clear();
                let path_back_again = self.pt_1();
                println!("Path back AGAIN: {}", path_back_again);
        */

        first_path + path_back + path_back_again
    }
}
